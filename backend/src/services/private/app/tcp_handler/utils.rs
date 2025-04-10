use futures_util::lock::Mutex;
use std::{net::SocketAddrV4, sync::Arc};

use actix::{Addr, AsyncContext, Context};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::{engine::{defs::fncodes::FNCODE, lib::MatrixCommand}, utils::configs::tcp_comunication_settings};

use super::{
    super::{
        messages::{ClosedByRemotePeer, MatrixReady, StreamFailed},
        schemas::MatrixStates,
    },  tcp_handler::TcpStreamActor
};

pub async fn process_response(
    not_timedout: Result<usize, std::io::Error>,
    socket: SocketAddrV4,
    ctx_addr: Addr<TcpStreamActor>,
    buffer: [u8; 128],
    mut states: MatrixStates,
    cmd: MatrixCommand,
    stream: Arc<Mutex<TcpStream>>,
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
                        states.set_changes(cmd);
                        let message = MatrixReady { socket, states, };
                        ctx_addr.do_send(message);
                    } else {
                        TcpStreamActor::read_states(ctx_addr, socket, stream).await;
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


pub fn command_polling(act: &mut TcpStreamActor, ctx: &mut Context<TcpStreamActor>){
    if !act.commands_queue.is_empty() {
        let cmd = act.commands_queue.pop_back().unwrap();
        let stream = act.stream.as_mut().unwrap().clone();
        let ctx_addr = ctx.address().clone();
        let socket = act.stream_socket;
        let states = act.machine_states.as_mut().unwrap().clone();
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
                tokio::time::timeout(tcp_comunication_settings::get_read_timeout(), stream_guard.read(&mut buffer)).await
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