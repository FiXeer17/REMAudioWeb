use crate::{
    engine::{defs::{datas, errors::Error}, lib::MatrixCommand},
    services::private::app::{messages, tcp_manager},
};
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use super::{
    messages::{ClosedByRemotePeer, MatrixReady, SetCommand, StreamFailed},
    schemas::{SetState, StreamError},
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
pub struct WsSession {
    pub hb: Instant,
    pub srv: Addr<tcp_manager::TcpStreamsManager>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub addr: Addr<WsSession>,
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                let address = ctx.address();
                act.srv.do_send(Disconnect { addr: address });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
    fn on_connect(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        self.srv.do_send(messages::Connect { addr, socket: None });
    }
}
impl WsSession {
    pub fn handle_text(&mut self, text: String) -> Result<MatrixCommand, Error> {
        let serialized: SetState = serde_json::from_str(&text).unwrap();
        let rw = datas::rw::WRITE.to_string();
        return MatrixCommand::new_from_client(rw, serialized)
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}

impl Handler<messages::BroadcastMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: messages::BroadcastMessage, ctx: &mut Self::Context) -> Self::Result {
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
                match self.handle_text(text.to_string()) {
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
