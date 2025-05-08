use std::time::Instant;

use crate::services::private::app::schemas::MachineStates;
use crate::services::private::app::utils::HasStatesMessage;
use crate::services::private::socket::utils::Device;
use crate::utils::common::toast;
use actix::{ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use log::debug;

use super::super::messages::*;
use super::session::WsSession;
use super::utils::{check_channel, check_preset, HandleText};

impl Handler<StreamFailed> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: StreamFailed, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(toast(&msg.error.to_string()).to_string());
    }
}
impl Handler<ClosedByRemotePeer> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(toast(&msg.message.to_string()).to_string());
    }
}

impl Handler<ClosedByAdmin> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ClosedByAdmin, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(toast(&format!("{} closed by admin",msg.device.unwrap().to_string())).to_string());
    }
}

impl Handler<GeneralError> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: GeneralError, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(toast(&msg.error).to_string());
    }
}

impl Handler<GeneralConnectionError> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: GeneralConnectionError, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(toast(&msg.error).to_string());
        ctx.stop();
    }
}



// POST-MIDDLEWARE
impl Handler<DeviceReady> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: DeviceReady, ctx: &mut Self::Context) -> Self::Result {
        let states = msg.get_states();
        let message = match states{
            MachineStates::CameraStates(cs) => serde_json::to_string_pretty(&cs).unwrap(),
            MachineStates::MatrixStates(ms) => serde_json::to_string(&ms).unwrap()
        };
        ctx.text(message);
    }
}



impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now(); // updating heartbeat if recieve a ping from client
                ctx.pong(&msg); 
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now(); //updating heartbeat if recieve a pong response to a ping from client
            }
            ws::Message::Text(text) => {
                self.hb = Instant::now();
                let addr = ctx.address(); // deserialize_text detect the command type sent by the client.
                match self.deserialize_text(text.to_string()) {
                    HandleText::Command(cmd) => match cmd {
                        Ok(cmd) => {
                            let msg = SetCommand { command: cmd };
                            self.srv.do_send(SetMessage {
                                addr,
                                command: Commands::SetMatrixCommand(msg),
                            });
                        }
                        Err(e) => {
                            ctx.text(e.to_string());
                        }
                    },

                    HandleText::Recache => {
                        self.srv.do_send(SetMessage {
                            addr,
                            command: Commands::ReCache,
                        });
                    }
                    HandleText::Error(reason) => {
                        ctx.text(toast(&reason).to_string());
                    }
                    HandleText::SetVisibility(sv) => {
                        let sv_clone = sv.clone();
                        let channel = sv_clone.channel.unwrap().parse::<u8>().unwrap();
                        if check_channel(channel) {
                            self.srv.do_send(SetMessage {
                                addr,
                                command: Commands::SetVisibility(sv),
                            });
                        }
                    },
                    HandleText::SetChannelLabels(sl) => {
                        let sl_clone = sl.clone();
                        let channel = sl_clone.channel.unwrap().parse::<u8>().unwrap();
                        if check_channel(channel) {
                            self.srv.do_send(SetMessage {
                                addr,
                                command: Commands::SetChannelLabel(sl),
                            });
                        }
                    },
                    HandleText::SetPresetLabels(sl) => {
                        let sl_clone = sl.clone();
                        let index = sl_clone.index.unwrap().parse::<u8>().unwrap();
                        if check_preset(index, Device::Audio) {
                            self.srv.do_send(SetMessage {
                                addr,
                                command: Commands::SetPresetLabel(sl),
                            });
                        }
                    },
                }
            }
            ws::Message::Binary(_) => debug!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
