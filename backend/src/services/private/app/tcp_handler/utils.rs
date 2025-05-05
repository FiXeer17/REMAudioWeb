use actix_web::web::Data;
use futures_util::lock::Mutex;
use std::{net::SocketAddrV4, sync::Arc};

use actix::{Addr, AsyncContext, Context};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use log::{info, warn};
use crate::{
    audio_engine::{
        defs::{datas::io::SRC, fncodes::FNCODE},
        lib::{read_all_states, MatrixCommand},
    },
    configs::tcp_comunication_settings,
    services::{
        private::app::{
            messages::{GeneralError, SetCommand, SetHandlerState},
            schemas::SetAttributes,
            ws_session::session::WsSession,
        },
        public::{
            interfaces::{
                self, add_io_channels, retrieve_channel_labels, retrieve_preset_labels, retrieve_socketid_from_db, retrieve_visibility, update_channel_labels_in_db, update_channel_visibility, update_preset_labels_in_db
            },
            utils::{retrieve_all_channels, retrieve_all_presets},
        },
    },
    AppState,
};

use super::{
    super::{
        messages::{ClosedByRemotePeer, MatrixReady, StreamFailed},
        schemas::MatrixStates,
    },
    tcp_handler::TcpStreamActor,
};
/*
    This function deserialize the hex byte string recieved into a readable 
    command struct, if the command is a preset command then a full matrix read will be ran.
    Else set_changes will take care of updating the cache. 
 */
pub async fn process_response(
    not_timedout: Result<usize, std::io::Error>,
    socket: SocketAddrV4,
    ctx_addr: Addr<TcpStreamActor>,
    buffer: [u8; 128],
    mut states: MatrixStates,
    cmd: MatrixCommand,
    stream: Arc<Mutex<TcpStream>>,
    pgpool: Data<AppState>,
) {
    match not_timedout {
        Ok(n) => {
            if n == 0 {
                let message = ClosedByRemotePeer {
                    message: "Closed by remote peer.".to_string(),
                    socket,
                };
                ctx_addr.do_send(message);
            } else {
                let buffer = &buffer[..n];
                let converted = buffer
                    .iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect::<Vec<String>>();
                if converted.get(0) == Some(&"00".to_string()) {
                    if cmd.fcode != FNCODE::SCENE.to_string() {
                        states.set_changes(cmd); // set changes detect changes from the recieved command and update the cache.
                        let message = MatrixReady { socket, states };
                        ctx_addr.do_send(message);
                    } else {
                        TcpStreamActor::read_states(ctx_addr.clone(), socket, stream, pgpool).await; //TODO DELETE .clone() 
                        warn!("DEBUG PURPOSE, DELETE ROWS: 78,79,80 IN PRODUCTION");
                        states.set_changes(cmd); //TODO DELETE THIS LINE  
                        let message = MatrixReady { socket, states }; //TODO DELETE THIS LINE 
                        ctx_addr.do_send(message); //TODO DELETE THIS LINE 

                    }
                }
            }
        }
        Err(e) => {
            let message = StreamFailed {
                error: e.to_string(),
                socket,
            };
            ctx_addr.do_send(message);
        }
    }
}


/*
    This function extract and send a command from the command_queue,
    process_response fn will take care of converting the bytes buffer and 
    return a response to the WebSocket handler.
 */
pub fn command_polling(act: &mut TcpStreamActor, ctx: &mut Context<TcpStreamActor>) {
    if !act.commands_queue.is_empty() {
        let cmd = act.commands_queue.pop_back().unwrap();
        let stream = act.stream.as_mut().unwrap().clone();
        let ctx_addr = ctx.address().clone();
        let socket = act.stream_socket;
        let states = act.machine_states.as_mut().unwrap().clone();
        let pgpool = act.pgpool.clone();
        tokio::spawn(async move {
            let written_bytes = {
                let mut steram_guard = stream.lock().await;
                steram_guard.write(&cmd.to_byte_hex().unwrap()).await
            };

            if let Err(e) = written_bytes {
                ctx_addr.do_send(ClosedByRemotePeer {
                    message: e.to_string(),
                    socket,
                });
                return;
            }

            let mut buffer = [0; 128];

            let read_bytes = {
                let mut stream_guard = stream.lock().await;
                tokio::time::timeout(
                    tcp_comunication_settings::get_read_timeout(),
                    stream_guard.read(&mut buffer),
                )
                .await
            };

            match read_bytes {
                Ok(not_timedout) => {
                    process_response(
                        not_timedout,
                        socket,
                        ctx_addr,
                        buffer,
                        states,
                        cmd,
                        stream,
                        pgpool,
                    )
                    .await
                }
                Err(t) => {
                    let reason = format!("read error:{}", t.to_string());
                    let message = StreamFailed {
                        error: reason,
                        socket,
                    };
                    ctx_addr.do_send(message);
                }
            }
        });
    }
}

pub async fn add_channels(pgpool: Data<AppState>, socket: SocketAddrV4) {
    let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, socket).await else {
        warn!("Cannot retrieve socket_id from the database");
        return;
    };
    let Ok(channels) = retrieve_all_channels(&pgpool, socket_id).await else {
        warn!("Cannot retrieve channels from database");
        return;
    };
    if let None = channels {
        if let Err(_) = add_io_channels(&pgpool, socket_id).await {
            warn!("Cannot add io channels");
            return;
        };
    }
    info!("Channels added succesfully for socket:{}.",socket.to_string())
}
pub async fn add_presets(pgpool: Data<AppState>, socket: SocketAddrV4, device: String) {
    let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, socket).await else {
        warn!("Cannot retrieve socket_id from the database");
        return;
    };
    let Ok(presets) = retrieve_all_presets(&pgpool, socket_id).await else {
        warn!("Cannot retrieve presets from database");
        return;
    };
    if let None = presets {
        let Ok(_) = interfaces::add_presets(&pgpool, socket_id, device.clone()).await else {
            warn!("Cannot add io channels");
            return;
        };
    }
    info!("Presets added succesfully for device type: {}.",device)

}

impl TcpStreamActor {
    pub async fn read_states(
        ctx_addr: Addr<TcpStreamActor>,
        socket: SocketAddrV4,
        stream: Arc<Mutex<TcpStream>>,
        pgpool: Data<AppState>,
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
        let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, socket).await else{
            warn!("Cannot retrieve socket id from database");
            return;
        };

        let visibility = retrieve_visibility(&pgpool, &socket_id).await;
        let channel_labels = retrieve_channel_labels(&pgpool, &socket_id).await;
        let preset_labels = retrieve_preset_labels(&pgpool, &socket_id).await;

        if let Err(_) = visibility {
            ctx_addr.do_send(GeneralError {
                error: "cannot attach visibility.".to_string(),
                socket: Some(socket.clone()),
            });
            return;
        }
        if let Err(_) = channel_labels {
            ctx_addr.do_send(GeneralError {
                error: "cannot attach channel labels.".to_string(),
                socket: Some(socket.clone()),
            });
            return;
        }

        if let Err(_) = preset_labels {
            ctx_addr.do_send(GeneralError {
                error: "cannot attach preset labels.".to_string(),
                socket: Some(socket.clone()),
            });
            return;
        }

        let (i_visibility, o_visibility) = visibility.unwrap();
        let (i_labels, o_labels) = channel_labels.unwrap();
        let preset_labels = preset_labels.unwrap();

        let states = MatrixStates::new(
            responses,
            socket.to_string(),
            i_labels,
            o_labels,
            preset_labels,
            i_visibility,
            o_visibility,
        );

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
    /*
        If a matrix command type is recieved it will be pushed inside the commands queue,
        command_polling fn will take care of it.
     */
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
            let pgpool = self.pgpool.clone();
            tokio::spawn(async move {
                TcpStreamActor::read_states(ctx_addr, socket, stream, pgpool).await;
            });
        }
    }
    pub fn handle_set_visibility_command(
        &mut self,
        sv: SetAttributes,
        pgpool: actix_web::web::Data<AppState>,
        addr: Addr<WsSession>,
        selfaddr: Addr<TcpStreamActor>,
    ) {
        let relative_identifier = sv.channel.unwrap().parse::<i32>().unwrap();
        let visibility = sv.value.parse::<bool>().unwrap();
        let stream_socket = self.stream_socket.clone();
        let states = self.machine_states.clone();

        let pgpool_clone = pgpool.clone();
        let io_clone = sv.io.clone().unwrap();
        let addr_clone = addr.clone();

        tokio::spawn(async move {
            let socket_id = retrieve_socketid_from_db(&pgpool, stream_socket).await;
            if socket_id.is_err() {
                addr_clone.do_send(GeneralError {
                    error: "cannot retrieve socket id in database".to_string(),
                    socket: Some(stream_socket),
                });
                warn!("Cannot retrieve socket id from the database");
                return;
            }
            let result = update_channel_visibility(
                &pgpool_clone,
                socket_id.unwrap(),
                relative_identifier,
                visibility,
                io_clone,
            )
            .await;
            if let Err(_) = result {
                addr_clone.do_send(GeneralError {
                    error: "cannot update channel visibility in database".to_string(),
                    socket: Some(stream_socket),
                });
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                if sv.io.unwrap() == SRC::INPUT.to_label() {
                    if !states.i_visibility.is_empty() {
                        states
                            .i_visibility
                            .insert(relative_identifier as u32, visibility);
                    }
                } else {
                    if !states.o_visibility.is_empty() {
                        states
                            .o_visibility
                            .insert(relative_identifier as u32, visibility);
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
    pub fn handle_set_channel_labels_command(
        &mut self,
        sv: SetAttributes,
        pgpool: actix_web::web::Data<AppState>,
        addr: Addr<WsSession>,
        selfaddr: Addr<TcpStreamActor>,
    ) {
        let relative_identifier = sv.channel.unwrap().parse::<i32>().unwrap();
        let label = sv.value;
        let stream_socket = self.stream_socket.clone();
        let states = self.machine_states.clone();

        let pgpool_clone = pgpool.clone();
        let io_clone = sv.io.clone().unwrap();
        let addr_clone = addr.clone();

        tokio::spawn(async move {
            let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, stream_socket).await else{
                addr_clone.do_send(GeneralError {
                    error: "cannot retrieve socket id in database".to_string(),
                    socket: Some(stream_socket),
                });
                warn!("Cannot retrieve socket id from the database");
                return;
            };
            let result = update_channel_labels_in_db(
                &pgpool_clone,
                socket_id,
                relative_identifier,
                label.clone(),
                io_clone,
            )
            .await;
            if let Err(_) = result {
                addr_clone.do_send(GeneralError {
                    error: "cannot update channel label in database".to_string(),
                    socket: Some(stream_socket),
                });
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                if sv.io.unwrap() == SRC::INPUT.to_label() {
                    if !states.i_labels.is_empty() {
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
    pub fn handle_set_preset_labels_command(
        &mut self,
        sv: SetAttributes,
        pgpool: actix_web::web::Data<AppState>,
        addr: Addr<WsSession>,
        selfaddr: Addr<TcpStreamActor>,
    ) {
        let relative_identifier = sv.index.unwrap().parse::<i32>().unwrap();
        let label = sv.value;
        let stream_socket = self.stream_socket.clone();
        let states = self.machine_states.clone();

        let pgpool_clone = pgpool.clone();
        let addr_clone = addr.clone();

        tokio::spawn(async move {
            let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, stream_socket).await else{
                addr_clone.do_send(GeneralError {
                    error: "cannot retrieve socket id in database".to_string(),
                    socket: Some(stream_socket),
                });
                warn!("Cannot retrieve socket id from the database");
                return;
            };

            let result = update_preset_labels_in_db(
                &pgpool_clone,
                socket_id,
                relative_identifier,
                label.clone(),
            )
            .await;

            if let Err(_) = result {
                addr_clone.do_send(GeneralError {
                    error: "cannot update channel label in database".to_string(),
                    socket: Some(stream_socket),
                });
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                if !states.preset_labels.is_empty(){
                    states.preset_labels.insert(relative_identifier as u32, label);
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

pub mod errors {
    #[derive(Clone, Debug)]
    pub enum Error {
        InvalidSrc,
        InvalidChannel,
        InvalidValue,
    }
}
