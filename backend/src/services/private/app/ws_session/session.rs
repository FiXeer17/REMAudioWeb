use super::utils::HandleText;
use crate::{
    engine::{
        defs::{datas, sections::Sections},
        lib::MatrixCommand,
    },
    services::private::app::{
        messages::{self, Disconnect},
        schemas::{SetAttributes, SetState},
    },
    utils::configs::websocket_settings,
    AppState,
};
use actix::prelude::*;
use actix_web::web::Data;
use actix_web_actors::ws;
use std::{net::SocketAddrV4, str::FromStr, time::Instant};

use super::super::tcp_manager::tcp_manager::TcpStreamsManager;

pub struct WsSession {
    pub hb: Instant,
    pub srv: Addr<TcpStreamsManager>,
    pub socket: Option<SocketAddrV4>,
    pub pgpool: Data<AppState>,
    pub user_id: i32,
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
    pub fn deserialize_text(&self, text: String) -> HandleText {
        if text == String::from("recache") {
            return HandleText::Recache;
        }

        if let Ok(set_state) = serde_json::from_str::<SetState>(&text) {
            match Sections::from_str(&set_state.section) {
                Ok(section) => match section {
                    Sections::Visibility => return handle_visibility(set_state),
                    Sections::Command(_) => return handle_command(set_state),
                    Sections::Labels => return handle_label(set_state),
                },
                Err(e) => return HandleText::Error(e.to_string()),
            }
        }
        return HandleText::Error("invalid command".to_string())
    }
    fn on_connect(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        self.srv.do_send(messages::Connect {
            addr,
            socket: self.socket,
        });
    }
}

fn handle_visibility(set_state: SetState) -> HandleText {
    if set_state.io.is_none() {
        return HandleText::Error("io is none".to_string());
    };
    let Some(ch) = set_state.channel.clone() else {
        return HandleText::Error("channel is none".to_string());
    };
    let Some(value) = set_state.value.clone() else {
        return HandleText::Error("value is none".to_string());
    };
    let ch = ch.parse::<u32>();
    if ch.is_err() {
        return HandleText::Error("channel is invalid".to_string());
    };
    let value = value.parse::<bool>();
    if value.is_err() {
        return HandleText::Error("value is invalid".to_string());
    };

    let set_visibility = SetAttributes {
        io: set_state.io.unwrap(),
        channel: set_state.channel.unwrap(),
        value: set_state.value.unwrap(),
    };
    HandleText::SetVisibility(set_visibility)
}

//fn handle_label(set_state: SetState) -> HandleText{}
fn handle_command(set_state: SetState) -> HandleText {
    let rw = datas::rw::WRITE.to_string();
    HandleText::Command(MatrixCommand::new_from_client(rw, set_state))
}

fn handle_label(set_state: SetState) -> HandleText{
    let Some(io) = set_state.io else{
        return HandleText::Error("io is none".to_string());
    };
    let Some(ch) = set_state.channel.clone() else {
        return HandleText::Error("channel is none".to_string());
    };
    let Some(value) = set_state.value.clone() else {
        return HandleText::Error("value is none".to_string());
    };
    let Ok(_) = ch.parse::<u32>() else{
        return HandleText::Error("cannot parse channel".to_string());
    };
    HandleText::SetLabels(SetAttributes{io,channel:ch,value})
}   

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}
