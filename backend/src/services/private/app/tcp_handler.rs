use super::{
    messages::{Connect, MatrixReady, StreamFailed, StreamStarted},
    schemas::MatrixStates,
    tcp_manager::TcpStreamsManager,
};

use crate::{
    engine::lib::{read_all_states, MatrixCommand},
    services::private::app::messages::ClosedByRemotePeer,
};
use actix::{Actor, ActorContext, Addr, AsyncContext, Context, Handler};
use futures_util::lock::Mutex;
use std::{collections::VecDeque, net::SocketAddrV4, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const COMMAND_DELAY: Duration = Duration::from_millis(201);
const RECONNECT_DELAY: Duration = Duration::from_secs(1);
const READ_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRIES: u8 = 3;

pub struct TcpStreamActor {
    pub stream_socket: SocketAddrV4,
    pub tcp_manager: Addr<TcpStreamsManager>,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
    pub commands_queue: VecDeque<MatrixCommand>,
    pub machine_states: Option<MatrixStates>,
}

impl TcpStreamActor {
    pub fn new(stream_socket: SocketAddrV4, tcp_manager: Addr<TcpStreamsManager>) -> Self {
        Self {
            stream_socket,
            tcp_manager,
            stream: None,
            commands_queue: VecDeque::new(),
            machine_states: None,
        }
    }
    pub async fn read_states(
        ctx_addr: Addr<TcpStreamActor>,
        socket: SocketAddrV4,
        stream: Arc<Mutex<TcpStream>>,
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
                tokio::time::timeout(READ_TIMEOUT, stream.read(&mut buffer)).await
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
            tokio::time::sleep(COMMAND_DELAY).await;
        }
        let states = MatrixStates::new(responses);
        ctx_addr.do_send(MatrixReady { states, socket });
    }
}

impl Actor for TcpStreamActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let socket = self.stream_socket.clone();
        let ctx_address = ctx.address().clone();
        tokio::spawn(async move {
            let mut retries: u8 = 0;
            while retries <= MAX_RETRIES {
                match TcpStream::connect(socket.to_string()).await {
                    Ok(tcp_stream) => {
                        tcp_stream.set_nodelay(false).unwrap();
                        let message = StreamStarted { tcp_stream };
                        ctx_address.do_send(message);
                        break;
                    }
                    Err(e) => {
                        if retries == MAX_RETRIES {
                            let message = StreamFailed {
                                socket,
                                error: e.to_string(),
                            };
                            ctx_address.do_send(message);
                            println!("cannot create tcp stream, closing...");
                            return;
                        } else {
                            retries += 1;
                            tokio::time::sleep(RECONNECT_DELAY).await;
                        }
                    }
                };
            }
        });
    }
}

impl Handler<StreamStarted> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: StreamStarted, ctx: &mut Self::Context) -> Self::Result {
        let socket = self.stream_socket.clone();
        let stream = Arc::new(Mutex::new(msg.tcp_stream));
        let ctx_addr = ctx.address().clone();
        self.stream = Some(stream.clone());

        tokio::spawn(async move {
            TcpStreamActor::read_states(ctx_addr, socket, stream).await;
        });

        ctx.run_interval(COMMAND_DELAY, |act, ctx| {
            if !act.commands_queue.is_empty() {
                let cmd = act.commands_queue.pop_back();
                let stream = act.stream.as_mut().unwrap().clone();
                let ctx_addr = ctx.address().clone();
                let socket = act.stream_socket;
                tokio::spawn(async move {
                    let written_bytes = {
                        let mut steram_guard = stream.lock().await;
                        steram_guard
                            .write(&cmd.unwrap().to_byte_hex().unwrap())
                            .await
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
                        tokio::time::timeout(READ_TIMEOUT, stream_guard.read(&mut buffer)).await
                    };

                    match read_bytes {
                        Ok(Ok(n)) => {
                            if n == 0 {
                                let message = ClosedByRemotePeer {
                                    message: "Closed by remote peer.".to_string(),
                                    socket,
                                };
                                ctx_addr.do_send(message);
                            }
                        }
                        Ok(Err(e)) => {
                            let message = StreamFailed {
                                error: e.to_string(),
                                socket,
                            };
                            ctx_addr.do_send(message);
                        }
                        Err(e) => {
                            let reason = format!("read error:{}", e.to_string());
                            let message = StreamFailed {
                                error: reason,
                                socket,
                            };
                            ctx_addr.do_send(message);
                        }
                    }
                });
            }
        });
    }
}

impl Handler<StreamFailed> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: StreamFailed, ctx: &mut Self::Context) -> Self::Result {
        self.tcp_manager.do_send(msg);
        ctx.stop();
    }
}
impl Handler<ClosedByRemotePeer> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, ctx: &mut Self::Context) -> Self::Result {
        self.tcp_manager.do_send(msg);
        ctx.stop();
    }
}

impl Handler<MatrixReady> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: MatrixReady, _: &mut Self::Context) -> Self::Result {
        self.machine_states = Some(msg.states.clone());
        self.tcp_manager.do_send(msg);
    }
}

impl Handler<Connect> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        if self.machine_states.is_none(){
            return ;
        }
        let states = self.machine_states.clone().unwrap();
        let message = MatrixReady {
            socket: msg.socket.unwrap(),
            states,
        };
        self.tcp_manager.do_send(message);
    }
}
