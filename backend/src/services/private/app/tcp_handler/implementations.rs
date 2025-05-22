use actix_web::web::Data;
use futures_util::lock::Mutex;
use std::{net::SocketAddrV4, sync::Arc};

use crate::{
    configs::tcp_comunication_settings,
    engines::{
        audio_engine::{
            defs::{datas::io::SRC, errors::Error},
            lib::{read_all_states, MatrixCommand},
        },
        video_engine::{self, camera_presets_lib::read_preset},
    },
    services::{
        private::{
            app::{
                messages::{
                    CameraReady, DeviceReady, GeneralError, SetCameraCommand, SetHandlerState,
                    SetMatrixCommand,
                },
                schemas::{CameraStates, DeviceCommnd, MachineStates, SetAttributes},
                ws_session::session::WsSession,
            },
            socket::utils::Device,
        },
        public::interfaces::{
            retrieve_channel_labels, retrieve_preset_labels, retrieve_socket_from_db,
            retrieve_socketid_from_db, retrieve_visibility, update_channel_labels_in_db,
            update_channel_visibility, update_preset_labels_in_db,
        },
    },
    AppState,
};
use actix::{Addr, AsyncContext, Context};
use log::warn;
use tokio::{
    net::TcpStream,
};

use super::{
    super::{
        messages::{ClosedByRemotePeer, MatrixReady, StreamFailed},
        schemas::MatrixStates,
    },
    tcp_handler::TcpStreamActor,
    utils::send_matrix_command,
};

impl TcpStreamActor {
    pub async fn read_audio_states(
        ctx_addr: Addr<TcpStreamActor>,
        socket: SocketAddrV4,
        stream: Arc<Mutex<TcpStream>>,
        pgpool: Data<AppState>,
    ) {
        let commands = read_all_states().unwrap();
        let buffer = [0u8; 128];
        let mut responses: Vec<MatrixCommand> = Vec::new();

        for command in commands {
            let mut retries = 0;
            let cmd_from_buffer: Result<Result<MatrixCommand, Error>, ()> = loop {
                let Ok(buffer) = send_matrix_command(command.clone(), stream.clone(), ctx_addr.clone(), buffer, socket).await else {break Err(());};

                match MatrixCommand::try_from(&buffer[..]) {
                    Ok(cmd) => break Ok(Ok(cmd)),
                    Err(err) => {
                        warn!("Command conversion failed at attempt {}, with buffer content: {:?}",retries,buffer);
                        if retries < tcp_comunication_settings::get_max_read_retries() {
                            retries += 1;
                            tokio::time::sleep(tcp_comunication_settings::get_command_delay()).await;
                            continue;
                        }
                        break Ok(Err(err));
                    }
                }
            };

            if cmd_from_buffer.is_err() {
                return;
            }

            if let Ok(Err(_)) = cmd_from_buffer {
                warn!("Cannot convert buffer in matrix command");
                ctx_addr.do_send(StreamFailed {
                    socket,
                    error: "error occurred on matrix".to_string(),
                });
                return;
            }
            responses.push(cmd_from_buffer.unwrap().unwrap());
            tokio::time::sleep(tcp_comunication_settings::get_command_delay()).await;
        }
        let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, socket).await else {
            warn!("Cannot retrieve socket id from database");
            return;
        };

        let visibility = retrieve_visibility(&pgpool, &socket_id).await;
        let channel_labels = retrieve_channel_labels(&pgpool, &socket_id).await;
        let preset_labels = retrieve_preset_labels(&pgpool, &socket_id).await;

        if let Err(_) = visibility {
            warn!("Cannot retrieve visibility");
            ctx_addr.do_send(GeneralError {
                error: "error occurred on matrix".to_string(),
                socket: Some(socket.clone()),
            });
            return;
        }
        if let Err(_) = channel_labels {
            warn!("Cannot retrieve channel labels.");
            ctx_addr.do_send(GeneralError {
                error: "error occurred on matrix".to_string(),
                socket: Some(socket.clone()),
            });
            return;
        }

        if let Err(_) = preset_labels {
            warn!("Cannot retrieve preset labels.");
            ctx_addr.do_send(GeneralError {
                error: "error occurred on matrix".to_string(),
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

        ctx_addr
            .clone()
            .do_send(DeviceReady::MatrixReady(MatrixReady { states, socket }));
    }

    pub async fn read_video_states(
        ctx_addr: Addr<TcpStreamActor>,
        socket: SocketAddrV4,
        stream: Arc<Mutex<TcpStream>>,
        pgpool: Data<AppState>,
    ) {
        let Ok(sock) = retrieve_socket_from_db(&pgpool, socket).await else {
            warn!("Cannot retrieve socket id from db");
            ctx_addr.do_send(GeneralError {
                socket: Some(socket),
                error: "error occurred on camera".to_string(),
            });
            return;
        };

        let Ok(preset_labels) = retrieve_preset_labels(&pgpool, &sock.id.unwrap()).await else {
            warn!("Cannot retrieve preset labels from db");
            ctx_addr.do_send(GeneralError {
                socket: Some(socket),
                error: "error occurred on camera".to_string(),
            });
            return;
        };
        let current_preset = read_preset(stream).await;
        if let Err(e) = current_preset {
            match e {
                video_engine::defs::errors::Error::ClosedByRemotePeer => {
                    let mess = ClosedByRemotePeer {
                        socket,
                        message: "error occurred on camera".to_string(),
                    };
                    ctx_addr.do_send(mess);
                    return;
                }
                video_engine::defs::errors::Error::InvalidPreset => {
                    let mess = StreamFailed {
                        socket,
                        error: "error occurred on camera".to_string(),
                    };
                    ctx_addr.do_send(mess);
                    return;
                }
                _ => unreachable!(),
            }
        }
        let current_preset = current_preset.unwrap();

        let states = CameraStates::new(sock.socket, preset_labels, current_preset);
        ctx_addr.do_send(DeviceReady::CameraReady(CameraReady { states, socket }));
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
                            match states {
                                MachineStates::MatrixStates(states) => {
                                    act.tcp_manager.do_send(DeviceReady::MatrixReady(
                                        MatrixReady {
                                            socket: act.stream_socket,
                                            states: states.clone(),
                                        },
                                    ));
                                }
                                MachineStates::CameraStates(states) => {
                                    act.tcp_manager.do_send(DeviceReady::CameraReady(
                                        CameraReady {
                                            socket: act.stream_socket,
                                            states: states.clone(),
                                        },
                                    ));
                                }
                            }
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
    pub fn handle_set_matrix_command(&mut self, sc: SetMatrixCommand) {
        self.commands_queue
            .push_front(DeviceCommnd::MatrixCommand(sc.command));
    }
    pub fn handle_set_camera_command(&mut self, sc: SetCameraCommand) {
        self.commands_queue
            .push_front(DeviceCommnd::CameraCommand(sc.command));
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
                TcpStreamActor::read_audio_states(ctx_addr, socket, stream, pgpool).await;
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
                    error: "error occurred on matrix".to_string(),
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
                warn!("Cannot update channel visibility in database");
                addr_clone.do_send(GeneralError {
                    error: "error occurred on matrix".to_string(),
                    socket: Some(stream_socket),
                });
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                let MachineStates::MatrixStates(states) = states else {
                    return;
                };
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
                selfaddr.do_send(DeviceReady::MatrixReady(MatrixReady {
                    socket: stream_socket,
                    states: states_clone,
                }));
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
            let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, stream_socket).await else {
                addr_clone.do_send(GeneralError {
                    error: "error occurred on matrix".to_string(),
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
                warn!("Cannot update channel label in database");
                addr_clone.do_send(GeneralError {
                    error: "error occurred on matrix".to_string(),
                    socket: Some(stream_socket),
                });
                return;
            }
            if let Some(states) = states.clone().as_mut() {
                let MachineStates::MatrixStates(states) = states else {
                    return;
                };

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
                selfaddr.do_send(DeviceReady::MatrixReady(MatrixReady {
                    socket: stream_socket,
                    states: states_clone,
                }));
            }
        });
    }
    pub fn handle_set_preset_labels_command(
        &mut self,
        sv: SetAttributes,
        pgpool: actix_web::web::Data<AppState>,
        addr: Addr<WsSession>,
        selfaddr: Addr<TcpStreamActor>,
        device: Device,
    ) {
        let relative_identifier = sv.index.unwrap().parse::<i32>().unwrap();
        let label = sv.value;
        let stream_socket = self.stream_socket.clone();
        let states = self.machine_states.clone();

        let pgpool_clone = pgpool.clone();
        let addr_clone = addr.clone();

        tokio::spawn(async move {
            let Ok(socket_id) = retrieve_socketid_from_db(&pgpool, stream_socket).await else {
                addr_clone.do_send(GeneralError {
                    error: format!("error occurred on {}", device.to_string()),
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
                warn!("Cannot update preset label indatabase");
                addr_clone.do_send(GeneralError {
                    error: format!("error occurred on {}", device.to_string()),
                    socket: Some(stream_socket),
                });
                return;
            }
            if let Some(mut states) = states {
                if let Some(inner) = states.as_mut_trait() {
                    let preset_labels = inner.preset_labels_mut();
                    if !preset_labels.is_empty() {
                        preset_labels.insert(relative_identifier as u32, label);
                    }
                }

                match states {
                    MachineStates::MatrixStates(ms) => {
                        selfaddr.do_send(DeviceReady::MatrixReady(MatrixReady {
                            socket: stream_socket,
                            states: ms,
                        }))
                    }
                    MachineStates::CameraStates(cs) => {
                        selfaddr.do_send(DeviceReady::CameraReady(CameraReady {
                            socket: stream_socket,
                            states: cs,
                        }))
                    }
                }
            }
        });
    }
}
