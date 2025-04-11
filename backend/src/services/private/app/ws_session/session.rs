use super::utils::{deserialize_text, HandleText};
use crate::{
    services::{
        private::app::{
            messages::{self, Disconnect},
            schemas::MatrixStates,
        },
        public::utils::Channel,
    },
    utils::configs::websocket_settings,
};
use actix::prelude::*;
use actix_web_actors::ws;
use std::{collections::HashMap, net::SocketAddrV4, time::Instant};

use super::super::tcp_manager::tcp_manager::TcpStreamsManager;

pub struct WsSession {
    pub hb: Instant,
    pub srv: Addr<TcpStreamsManager>,
    pub socket: Option<SocketAddrV4>,
    pub user_id: i32,
    pub i_channels: Vec<Channel>,
    pub o_channels: Vec<Channel>,
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(websocket_settings::get_heartbeat_interval(), |act, ctx| {
            if Instant::now().duration_since(act.hb) > websocket_settings::get_client_timeout() {
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
        self.srv.do_send(messages::Connect {
            addr,
            socket: self.socket,
        });
    }
}

impl WsSession {
    pub fn handle_text(&mut self, text: String, _addr: Addr<WsSession>) -> HandleText {
        deserialize_text(text)
    }

    pub fn attach_channel_visibility(&self, states: &mut MatrixStates) -> MatrixStates {
        let mut i_visibility_map: HashMap<u32, bool> = HashMap::new();
        let mut o_visibility_map: HashMap<u32, bool> = HashMap::new();
        self.i_channels.iter().enumerate().for_each(|(i, channel)| {
            let index = i + 1;
            i_visibility_map
                .entry(index as u32)
                .or_insert(channel.visible);
        });
        self.o_channels.iter().enumerate().for_each(|(i, channel)| {
            let index = i + 1;
            o_visibility_map
                .entry(index as u32)
                .or_insert(channel.visible);
        });

        states.i_visibility = Some(i_visibility_map);
        states.o_visibility = Some(o_visibility_map);
        states.clone()
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}
