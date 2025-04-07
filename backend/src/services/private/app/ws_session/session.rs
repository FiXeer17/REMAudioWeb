use crate::{
    engine::{defs::datas, lib::MatrixCommand},
    services::private::app::messages::{self, Commands, Disconnect, SetMessage}, utils::configs::WebsocketEnv,
};
use actix::prelude::*;
use actix_web_actors::ws;
use std::{net::SocketAddrV4, time::Instant};
use super::utils::HandleText;

use super::{
    super::schemas::SetState, super::tcp_manager::tcp_manager::TcpStreamsManager,
};

pub struct WsSession {
    pub hb: Instant,
    pub srv: Addr<TcpStreamsManager>,
    pub socket: Option<SocketAddrV4>,
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(WebsocketEnv::get_heartbeat_interval(), |act, ctx| {
            if Instant::now().duration_since(act.hb) > WebsocketEnv::get_client_timeout() {
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
        self.srv.do_send(messages::Connect { addr, socket:self.socket });
    }
}

impl WsSession {
    pub fn handle_text(&mut self, text: String, addr: Addr<WsSession>) -> HandleText {
        if text == String::from("recache"){
            self.srv.do_send(SetMessage{addr,command:Commands::ReCache});
            return HandleText::Recache;
        }
        let serialized: SetState = serde_json::from_str(&text).unwrap();
        let rw = datas::rw::WRITE.to_string();
        return HandleText::Command(MatrixCommand::new_from_client(rw, serialized));
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}

