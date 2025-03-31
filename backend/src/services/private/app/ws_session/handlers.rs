use std::time::Instant;

use crate::services::private::app::schemas::StreamError;
use actix::{ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;

use super::super::messages::*;
use super::session::WsSession;
use super::utils::HandleText;

impl Handler<BroadcastMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.message);
    }
}

impl Handler<StreamFailed> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: StreamFailed, ctx: &mut Self::Context) -> Self::Result {
        let failed_socket = msg.socket.to_string();
        let message = StreamError {
            fail_reason: msg.error,
            at_socket: failed_socket,
        };

        ctx.text(serde_json::to_string_pretty(&message).unwrap());
        ctx.stop();
    }
}
impl Handler<ClosedByRemotePeer> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, ctx: &mut Self::Context) -> Self::Result {
        let failed_socket = msg.socket.to_string();
        let message = StreamError {
            fail_reason: msg.message,
            at_socket: failed_socket,
        };
        ctx.text(serde_json::to_string_pretty(&message).unwrap());
        ctx.stop();
    }
}

impl Handler<MatrixReady> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: MatrixReady, ctx: &mut Self::Context) -> Self::Result {
        let message = serde_json::to_string_pretty(&msg.states).unwrap();
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
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                self.hb = Instant::now();
                match self.handle_text(text.to_string(), ctx.address()) {
                    HandleText::Command(cmd) => match cmd {
                        Ok(cmd) => {
                            let msg = SetCommand {
                                addr: ctx.address(),
                                command: cmd,
                            };
                            self.srv.do_send(msg);
                        }
                        Err(e) => {
                            ctx.text(e.to_string());
                        }
                    },
                    HandleText::Recache => println!("recaching..."),
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
