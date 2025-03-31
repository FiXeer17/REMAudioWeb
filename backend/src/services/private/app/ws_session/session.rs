use crate::{
    engine::{defs::{datas, errors::Error}, lib::MatrixCommand},
    services::private::app::messages::{self, Disconnect},
};
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;
use super::configs::*;

use super::{
    super::schemas::SetState, super::tcp_manager::tcp_manager::TcpStreamsManager,
};

pub struct WsSession {
    pub hb: Instant,
    pub srv: Addr<TcpStreamsManager>,
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

