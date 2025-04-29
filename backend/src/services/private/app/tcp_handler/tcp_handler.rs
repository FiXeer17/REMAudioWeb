use super::super::{
    messages::{MatrixReady, StreamFailed, StreamStarted},
    schemas::MatrixStates,
};

use crate::{
    engine::lib::{read_all_states, MatrixCommand},
    services::{private::app::{
        messages::{ClosedByRemotePeer, GeneralError, SetCommand, SetHandlerState}, schemas::SetAttributes, tcp_manager::tcp_manager::TcpStreamsManager, ws_session::session::WsSession
    }, public::{interfaces::{retrieve_labels, retrieve_socketid_from_db, retrieve_visibility, update_channel_visibility, update_labels_in_db}, utils::SRC}},
    configs::tcp_comunication_settings, AppState,
};
use actix::{Actor, Addr, AsyncContext, Context, SpawnHandle};
use actix_web::web::Data;
use futures_util::lock::Mutex;
use std::{collections::VecDeque, net::SocketAddrV4, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct TcpStreamActor {
    pub stream_socket: SocketAddrV4,
    pub tcp_manager: Addr<TcpStreamsManager>,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
    pub commands_queue: VecDeque<MatrixCommand>,
    pub machine_states: Option<MatrixStates>,
    pub cmd_poller: Option<SpawnHandle>,
    pub owner: Option<SpawnHandle>,
    pub pgpool:Data<AppState>,
}

impl TcpStreamActor {
    pub fn new(stream_socket: SocketAddrV4, tcp_manager: Addr<TcpStreamsManager>,pgpool: actix_web::web::Data<AppState>) -> Self {
        Self {
            stream_socket,
            tcp_manager,
            stream: None,
            commands_queue: VecDeque::new(),
            machine_states: None,
            cmd_poller: None,
            owner: None,
            pgpool
        }
    }
    pub async fn read_states(
        ctx_addr: Addr<TcpStreamActor>,
        socket: SocketAddrV4,
        stream: Arc<Mutex<TcpStream>>,
        pgpool:Data<AppState>
    ) {
        let commands = read_all_states().unwrap();
        let mut buffer = [0u8; 128];
        let mut responses: Vec<MatrixCommand> = Vec::new();

        for command in commands {
            let cmd = command.to_byte_hex().unwrap();

            let written_bytes = {
                let mut stream = stream.lock().await;
                stream.write(&cmd[..]).await
            };

            if let Err(e) = written_bytes {
                ctx_addr.do_send(ClosedByRemotePeer {
                    message: e.to_string(),
                    socket,
                });
                return;
            }

            let read_bytes = {
                let mut stream = stream.lock().await;
                tokio::time::timeout(
                    tcp_comunication_settings::get_read_timeout(),
                    stream.read(&mut buffer),
                )
                .await
            };
            if let Ok(Ok(n)) = read_bytes {
                if n == 0 {
                    ctx_addr.do_send(ClosedByRemotePeer {
                        message: "Closed by remote peer".to_string(),
                        socket,
                    });
                    return;
                }
            }

            if let Err(e) = read_bytes {
                ctx_addr.do_send(StreamFailed {
                    socket,
                    error: e.to_string(),
                });
                return;
            }

            if let Ok(Err(e)) = read_bytes {
                let message = StreamFailed {
                    error: e.to_string(),
                    socket,
                };
                ctx_addr.do_send(message);
                return;
            }

            let read_bytes = read_bytes.unwrap();

            let buffer = &buffer[..read_bytes.unwrap()];
            let cmd_from_buffer = MatrixCommand::try_from(buffer);

            if let Err(e) = cmd_from_buffer {
                ctx_addr.do_send(StreamFailed {
                    socket,
                    error: e.to_string(),
                });
                return;
            }
            responses.push(cmd_from_buffer.unwrap());
            tokio::time::sleep(tcp_comunication_settings::get_command_delay()).await;
        }
        let socket_id = retrieve_socketid_from_db(&pgpool, socket).await;
        if socket_id.is_err(){
            println!("cannot retrieve socket id from database");
            return;
        }
        let visibility = retrieve_visibility(&pgpool, socket_id.as_ref().unwrap()).await;
        let labels= retrieve_labels(&pgpool, &socket_id.unwrap()).await;
        if let Err(_) = visibility{
            ctx_addr.do_send(GeneralError{error:"cannot attach visibility.".to_string(),socket:Some(socket.clone())});
            return;
        }
        if let Err(_) = labels{
            ctx_addr.do_send(GeneralError{error:"cannot attach labels.".to_string(),socket:Some(socket.clone())});
            return;
        }
        let (i_visibility,o_visibility) = visibility.unwrap();
        let (i_labels,o_labels) = labels.unwrap();

        let states = MatrixStates::new(responses, socket.to_string(),i_labels,o_labels,i_visibility,o_visibility);

        ctx_addr.clone().do_send(MatrixReady { states, socket });
    }

    pub fn set_handler_state(&mut self, state: Option<Addr<WsSession>>) {
        self.tcp_manager.do_send(SetHandlerState {
            socket: self.stream_socket,
            state,
        });
    }
    pub fn watch_inactive(&mut self, ctx: &mut Context<Self>, addr: Addr<WsSession>) {
        if self.owner.is_none() {
            self.set_handler_state(Some(addr));
        } else {
            ctx.cancel_future(self.owner.unwrap());
            self.owner = None;
        }
        self.owner = Some(ctx.run_interval(
            tcp_comunication_settings::get_inactivity_timeout(),
            |act, ctx| {
                if act.commands_queue.is_empty() {
                    if let Some(owner) = act.owner {
                        ctx.cancel_future(owner);
                        act.owner = None;
                        act.set_handler_state(None);
                        if let Some(states) = &act.machine_states {
                            act.tcp_manager.do_send(MatrixReady {
                                socket: act.stream_socket,
                                states: states.clone(),
                            });
                        }
                    }
                }
            },
        ));
    }
    pub fn handle_set_command(&mut self, sc: SetCommand) {
        self.commands_queue.push_front(sc.command);
    }
    pub fn handle_recache(&mut self, ctx: &mut Context<Self>) {
        if self.machine_states.is_some() {
            if let Some(poller) = self.cmd_poller {
                ctx.cancel_future(poller);
                self.cmd_poller = None;
            }
            let ctx_addr = ctx.address().clone();
            let socket = self.stream_socket.clone();
            let stream = self.stream.as_mut().unwrap().clone();
            let pgpool =self.pgpool.clone();
            tokio::spawn(async move {
                TcpStreamActor::read_states(ctx_addr, socket, stream,pgpool).await;
            });
        }
    }
    pub fn handle_set_visibility_command(&mut self, sv: SetAttributes, pgpool: actix_web::web::Data<AppState>, addr: Addr<WsSession>, selfaddr: Addr<TcpStreamActor>) {
        let relative_identifier = sv.channel.parse::<i32>().unwrap();
        let visibility = sv.value.parse::<bool>().unwrap();
        let stream_socket = self.stream_socket.clone();
        let states = self.machine_states.clone();
        
        let pgpool_clone = pgpool.clone();
        let io_clone = sv.io.clone();
        let addr_clone = addr.clone();
        
        tokio::spawn(async move {
            let socket_id = retrieve_socketid_from_db(&pgpool, stream_socket).await;
            if socket_id.is_err() {
                addr_clone.do_send(GeneralError{error: "cannot retrieve socket id in database".to_string(),socket:Some(stream_socket)});
                println!("cannot retrieve socket_id from the database");
                return;
            }
            let result = update_channel_visibility(&pgpool_clone, socket_id.unwrap(), relative_identifier, visibility, io_clone).await;
            if let Err(_) = result {
                addr_clone.do_send(GeneralError{error: "cannot update channel visibility in database".to_string(),socket:Some(stream_socket)});
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                if sv.io == SRC::INPUT.to_string() {
                    if !states.i_visibility.is_empty(){
                        states.i_visibility.insert(relative_identifier as u32, visibility);
                    }
                } else {
                    if !states.o_visibility.is_empty() {
                        states.o_visibility.insert(relative_identifier as u32, visibility);
                    }
                }
                
                let states_clone = states.clone();
                selfaddr.do_send(MatrixReady {
                    socket: stream_socket,
                    states: states_clone,
                });
            }

        });
    }

    pub fn handle_set_label_command(&mut self, sv: SetAttributes, pgpool: actix_web::web::Data<AppState>, addr: Addr<WsSession>, selfaddr: Addr<TcpStreamActor>) {
        let relative_identifier = sv.channel.parse::<i32>().unwrap();
        let label = sv.value;
        let stream_socket = self.stream_socket.clone();
        let states = self.machine_states.clone();
        
        let pgpool_clone = pgpool.clone();
        let io_clone = sv.io.clone();
        let addr_clone = addr.clone();
        
        tokio::spawn(async move {
            let socket_id = retrieve_socketid_from_db(&pgpool, stream_socket).await;
            if socket_id.is_err() {
                addr_clone.do_send(GeneralError{error: "cannot retrieve socket id in database".to_string(),socket:Some(stream_socket)});
                println!("cannot retrieve socket_id from the database");
                return;
            }
            let result = update_labels_in_db(&pgpool_clone, socket_id.unwrap(), relative_identifier, label.clone(), io_clone).await;
            if let Err(_) = result {
                addr_clone.do_send(GeneralError{error: "cannot update channel label in database".to_string(),socket:Some(stream_socket)});
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                if sv.io == SRC::INPUT.to_string() {
                    if !states.i_labels.is_empty(){
                        states.i_labels.insert(relative_identifier as u32, label);
                    }
                } else {
                    if !states.o_labels.is_empty() {
                        states.o_labels.insert(relative_identifier as u32, label);
                    }
                }
                
                let states_clone = states.clone();
                selfaddr.do_send(MatrixReady {
                    socket: stream_socket,
                    states: states_clone,
                });
            }

        });
    }
}

impl Actor for TcpStreamActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let socket = self.stream_socket.clone();
        let ctx_address = ctx.address().clone();
        tokio::spawn(async move {
            let mut retries: u8 = 0;
            while retries <= tcp_comunication_settings::get_max_connection_retries(){
                match tokio::time::timeout(
                    tcp_comunication_settings::get_connection_timeout(),
                    TcpStream::connect(socket.to_string()),
                )
                .await
                {
                    Ok(not_timedout) => match not_timedout {
                        Ok(tcp_stream) => {
                            tcp_stream.set_nodelay(false).unwrap();
                            let message = StreamStarted { tcp_stream };
                            ctx_address.do_send(message);
                            break;
                        }
                        Err(e) => {
                            if retries == tcp_comunication_settings::get_max_connection_retries() {
                                let message = StreamFailed {
                                    socket,
                                    error: e.to_string(),
                                };
                                ctx_address.do_send(message);
                                println!("cannot create tcp stream, closing...");
                                return;
                            } else {
                                retries += 1;
                                tokio::time::sleep(tcp_comunication_settings::get_reconnect_delay()).await;
                            }
                        }
                    },
                    Err(t) => {
                        if retries == tcp_comunication_settings::get_max_connection_retries() {
                            let message = StreamFailed {
                                socket,
                                error: t.to_string(),
                            };
                            ctx_address.do_send(message);
                            println!("cannot create tcp stream (elapsed), closing...");
                            return;
                        }
                        retries += 1;
                    }
                };
            }
        });
    }
}
