use actix_web::web::Data;
use futures_util::lock::Mutex;
use std::{net::SocketAddrV4, sync::Arc};

use crate::{
    configs::tcp_comunication_settings,
    engines::{
        audio_engine::{defs::fncodes::FNCODE, lib::MatrixCommand},
        video_engine::{defs::CameraCommand, status_codes_lib::successfull},
    },
    services::{
        private::app::{
            messages::{CameraReady, DeviceReady},
            schemas::{DeviceCommnd, MachineStates},
        },
        public::{
            interfaces::{self, add_io_channels, retrieve_socketid_from_db},
            utils::{retrieve_all_channels, retrieve_all_presets},
        },
    },
    AppState,
};
use actix::{Addr, AsyncContext, Context};
use log::{info, warn};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
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
    socket: SocketAddrV4,
    ctx_addr: Addr<TcpStreamActor>,
    mut states: MatrixStates,
    cmd: MatrixCommand,
    stream: Arc<Mutex<TcpStream>>,
    pgpool: Data<AppState>,
) {
    if cmd.fcode != FNCODE::SCENE.to_string() {
        states.set_changes(cmd); // set changes detect changes from the recieved command and update the cache.
        let message = MatrixReady { socket, states };
        ctx_addr.do_send(DeviceReady::MatrixReady(message));
    } else {
        TcpStreamActor::read_audio_states(ctx_addr, socket, stream, pgpool).await;
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
        match cmd {
            DeviceCommnd::MatrixCommand(mc) => handle_matrix_polling(act, ctx, mc),
            DeviceCommnd::CameraCommand(cc) => handle_camera_polling(act, ctx, cc),
        }
    }
}
pub fn handle_camera_polling(
    act: &mut TcpStreamActor,
    ctx: &mut Context<TcpStreamActor>,
    cmd: CameraCommand,
) {
    let stream = act.stream.as_mut().unwrap().clone();
    let ctx_addr = ctx.address().clone();
    let socket = act.stream_socket;
    let MachineStates::CameraStates(mut states) = act.machine_states.as_mut().unwrap().clone()
    else {
        return;
    };
    tokio::spawn(async move {
        let written_bytes = {
            let mut steram_guard = stream.lock().await;
            steram_guard.write(&cmd.cmd[..]).await
        };

        if let Err(_) = written_bytes {
            warn!("closed by remote peer on write");
            ctx_addr.do_send(ClosedByRemotePeer {
                message: "error occurred on camera".to_string(),
                socket,
            });
            return;
        }

        match successfull(stream).await {
            Ok(true) => {
                if cmd.fncode == crate::engines::video_engine::defs::fncodes::FNCODE::Preset {
                    let latest_preset = *cmd.cmd.get(5).unwrap() as i32;
                    states.current_preset = latest_preset;
                }
                let message = CameraReady {
                    socket,
                    states: states,
                };
                ctx_addr.do_send(DeviceReady::CameraReady(message));
            }
            Ok(false) => {
                let message = StreamFailed {
                    error: "error occurred on camera".to_string(),
                    socket,
                };
                ctx_addr.do_send(message);
            }
            Err(_) => {
                let message = StreamFailed {
                    error: "error occurred on camera".to_string(),
                    socket,
                };
                ctx_addr.do_send(message);
            }
        }
    });
}

pub fn handle_matrix_polling(
    act: &mut TcpStreamActor,
    ctx: &mut Context<TcpStreamActor>,
    cmd: MatrixCommand,
) {
    let stream = act.stream.as_mut().unwrap().clone();
    let ctx_addr = ctx.address().clone();
    let socket = act.stream_socket;

    let MachineStates::MatrixStates(states) = act.machine_states.as_mut().unwrap().clone() else {
        return;
    };
    let pgpool = act.pgpool.clone();
    if cmd.fcode == FNCODE::SCENE.to_string() {
        if act.machine_states.is_some() {
            if let Some(poller) = act.cmd_poller {
                ctx.cancel_future(poller);
                act.cmd_poller = None;
            }
        }
    }
    tokio::spawn(async move {
        let written_bytes = {
            let mut steram_guard = stream.lock().await;
            steram_guard.write(&cmd.to_byte_hex().unwrap()).await
        };

        if let Err(_) = written_bytes {
            warn!("closed by remote peer on write");
            ctx_addr.do_send(ClosedByRemotePeer {
                message: "error occurred on matrix".to_string(),
                socket,
            });
            return;
        }

        let mut buffer = [0; 128];
        let mut timeout = tcp_comunication_settings::get_read_timeout();
        if cmd.fcode == FNCODE::SCENE.to_string() {
            timeout = tcp_comunication_settings::get_preset_read_timeout();
        }

        let read_bytes = {
            let mut stream_guard = stream.lock().await;
            tokio::time::timeout(timeout, stream_guard.read(&mut buffer)).await
        };
        match read_bytes {
            Ok(Ok(lenght)) => {
                if lenght == 0 {
                    let message = ClosedByRemotePeer {
                        socket,
                        message: "Closed by remote peer".to_string(),
                    };
                    ctx_addr.do_send(message);
                    return;
                }
            }
            Ok(Err(e)) => {
                let message = StreamFailed {
                    socket,
                    error: e.to_string(),
                };
                ctx_addr.do_send(message);
                return;
            }
            _ => (),
        }
        process_response(socket, ctx_addr, states, cmd, stream, pgpool).await;
    });
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
    info!(
        "Channels added succesfully for socket:{}.",
        socket.to_string()
    )
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
            warn!("Cannot add presets");
            return;
        };
    }
    info!("Presets added succesfully for device type: {}.", device)
}

pub async fn send_matrix_command(
    command: MatrixCommand,
    stream: Arc<Mutex<TcpStream>>,
    ctx_addr: Addr<TcpStreamActor>,
    mut buffer: [u8; 128],
    socket: SocketAddrV4,
) -> Result<Vec<u8>, ()> {
    let cmd = command.to_byte_hex().unwrap();
    let written_bytes = {
        let mut stream = stream.lock().await;
        stream.write(&cmd).await
    };

    if let Err(_) = written_bytes {
        warn!("closed by remote peer on write");
        ctx_addr.do_send(ClosedByRemotePeer {
            message: "error occurred on matrix".to_string(),
            socket,
        });
        return Err(());
    }
    let timeout = tcp_comunication_settings::get_read_timeout();
    let read_bytes = {
        let mut stream = stream.lock().await;
        tokio::time::timeout(timeout, stream.read(&mut buffer)).await
    };
    if let Ok(Ok(n)) = read_bytes {
        if n == 0 {
            warn!("closed by remote peer on read (0 bytes)");
            ctx_addr.do_send(ClosedByRemotePeer {
                message: "error occurred on matrix".to_string(),
                socket,
            });
            return Err(());
        }
    }

    if let Err(_) = read_bytes {
        warn!("Time elapsed for response.");
        ctx_addr.do_send(StreamFailed {
            socket,
            error: "error occurred on matrix".to_string(),
        });
        return Err(());
    }

    if let Ok(Err(_)) = read_bytes {
        warn!("Cannot read response");
        let message = StreamFailed {
            error: "error occurred on matrix".to_string(),
            socket,
        };
        ctx_addr.do_send(message);
        return Err(());
    }

    let read_bytes = read_bytes.unwrap();

    Ok(buffer[..read_bytes.unwrap()].to_vec())
}

pub mod errors {
    #[derive(Clone, Debug)]
    pub enum Error {
        InvalidSrc,
        InvalidChannel,
        InvalidValue,
    }
}
