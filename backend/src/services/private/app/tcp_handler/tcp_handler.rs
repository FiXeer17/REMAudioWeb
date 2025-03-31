use super::super::{
        messages::{MatrixReady, StreamFailed, StreamStarted},
        schemas::MatrixStates,
    };

use super::configs::*;
use crate::{
    engine::lib::{read_all_states, MatrixCommand},
    services::private::app::{messages::ClosedByRemotePeer, tcp_manager::tcp_manager::TcpStreamsManager},
};
use actix::{Actor, Addr, AsyncContext, Context, SpawnHandle};
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
    pub cmd_poller: Option<SpawnHandle>
}

impl TcpStreamActor {
    pub fn new(stream_socket: SocketAddrV4, tcp_manager: Addr<TcpStreamsManager>) -> Self {
        Self {
            stream_socket,
            tcp_manager,
            stream: None,
            commands_queue: VecDeque::new(),
            machine_states: None,
            cmd_poller: None,
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
                match tokio::time::timeout(TIMEDOUT_TIME, TcpStream::connect(socket.to_string()))
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
                    },
                    Err(t) => {
                        if retries == MAX_RETRIES {
                            let message = StreamFailed {
                                socket,
                                error: t.to_string(),
                            };
                            ctx_address.do_send(message);
                            println!("cannot create tcp stream (elapsed), closing...");
                            return;
                        }
                        retries+=1;
                    }
                };
            }
        });
    }
}


