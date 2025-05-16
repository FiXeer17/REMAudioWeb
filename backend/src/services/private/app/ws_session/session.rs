use super::{
    text_handlers::{
        handle_channel_label, handle_matrix_command, handle_preset_label, handle_video_command, handle_visibility
    },
    utils::HandleText,
};
use crate::{
    configs::websocket_settings,
    engines::sections::Sections,
    services::private::app::{
        messages::{self, Disconnect},
        schemas::SetState,
    },
    AppState,
};
use actix::prelude::*;
use actix_web::web::Data;
use actix_web_actors::ws;
use log::info;
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
                info!("Websocket Client heartbeat failed, disconnecting!");
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
                    Sections::MatrixCommand(_) => return handle_matrix_command(set_state),
                    Sections::CameraCommand(_) => return handle_video_command(set_state, &section),
                    Sections::Visibility => return handle_visibility(set_state),
                    Sections::ChannelLabels => return handle_channel_label(set_state),
                    Sections::MatrixPresetLabels => return handle_preset_label(set_state),
                    Sections::CameraPresetLabels => return handle_preset_label(set_state),
                },
                Err(e) => return HandleText::Error(e.to_string()),
            }
        }
        return HandleText::Error("invalid command".to_string());
    }
    fn on_connect(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        self.srv.do_send(messages::Connect {
            addr,
            socket: self.socket,
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}
