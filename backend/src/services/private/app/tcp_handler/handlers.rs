use std::sync::Arc;

use actix::{ActorContext, AsyncContext, Handler};
use futures_util::lock::Mutex;
use log::info;

use crate::configs::tcp_comunication_settings;
use crate::services::private::app::schemas::MachineStates;
use crate::services::private::app::utils::HasStatesMessage;
use crate::services::private::socket::utils::Device;

use super::super::messages::*;
use super::tcp_handler::TcpStreamActor;
use super::utils::command_polling;

impl Handler<StreamStarted> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: StreamStarted, ctx: &mut Self::Context) -> Self::Result {
        info!("Starting new {} tcp handler actor",self.device_type.to_string().to_uppercase());
        let socket = self.stream_socket.clone();
        let stream = Arc::new(Mutex::new(msg.tcp_stream));
        let ctx_addr = ctx.address().clone();
        let pgpool = self.pgpool.clone();
        self.stream = Some(stream.clone());
        match self.device_type {
            Device::Audio => {
                tokio::spawn(async move {
                    TcpStreamActor::read_audio_states(ctx_addr, socket, stream, pgpool).await;
                });
            },
            Device::Video =>{
                tokio::spawn(async move {
                    TcpStreamActor::read_video_states(ctx_addr,socket, stream,pgpool).await;
                });
            }
        }
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

impl Handler<ClosedByAdmin> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, _msg: ClosedByAdmin, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<DeviceReady> for TcpStreamActor{
    type Result = ();
    fn handle(&mut self, msg: DeviceReady, ctx: &mut Self::Context) -> Self::Result {
        self.machine_states = Some(msg.get_states());
        self.tcp_manager.do_send(msg);
        if self.cmd_poller.is_none() {
            let cmd_poller = ctx.run_interval(
                tcp_comunication_settings::get_command_delay(),
                command_polling,
            );
            self.cmd_poller = Some(cmd_poller);
        }

    }
}


impl Handler<GeneralError> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: GeneralError, _ctx: &mut Self::Context) -> Self::Result {
        self.tcp_manager.do_send(msg);
    }
}

impl Handler<Connect> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        if self.machine_states.is_none() {
            return;
        }
        match self.machine_states.clone().unwrap(){
            MachineStates::MatrixStates(states) =>{
                let message = MatrixReady {
                    socket: msg.socket.unwrap(),
                    states,
                };
                self.tcp_manager.do_send(DeviceReady::MatrixReady(message));
            },
            MachineStates::CameraStates(_) =>()
        }
        
    }
}

impl Handler<SetMessage> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: SetMessage, ctx: &mut Self::Context) -> Self::Result {
        self.watch_inactive(ctx, msg.addr.clone());
        match msg.command {
            Commands::SetMatrixCommand(sc) => self.handle_set_command(sc),
            Commands::SetVisibility(sv) => {
                if self.machine_states.is_some() {
                    self.handle_set_visibility_command(
                        sv,
                        self.pgpool.clone(),
                        msg.addr,
                        ctx.address(),
                    );
                }
            }
            Commands::SetChannelLabel(sl) => {
                if self.machine_states.is_some() {
                    self.handle_set_channel_labels_command(sl, self.pgpool.clone(), msg.addr, ctx.address())
                }
            },
            Commands::SetPresetLabel(sl) => {
                if self.machine_states.is_some() {
                    self.handle_set_preset_labels_command(sl, self.pgpool.clone(), msg.addr, ctx.address())
                }
            }
            Commands::ReCache => self.handle_recache(ctx),
        }
    }
}
