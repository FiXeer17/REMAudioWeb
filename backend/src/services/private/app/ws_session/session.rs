use super::utils::HandleText;
use crate::{
    engine::{defs::datas, lib::MatrixCommand}, services::{
        private::app::{
            messages::{self, Disconnect},
            schemas::{MatrixStates, SetState, SetVisibility},
        },
        public::{interfaces, utils::SRC},
    }, utils::configs::websocket_settings, AppState
};
use actix::prelude::*;
use actix_web::web::{self, Data};
use actix_web_actors::ws;
use std::{collections::HashMap, net::SocketAddrV4, time::Instant};

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
    pub fn deserialize_text(&self,text:String)-> HandleText{
        if text == String::from("recache"){
            return HandleText::Recache;
        }
    
        if let Ok(set_state) = serde_json::from_str::<SetState>(&text){
            let rw = datas::rw::WRITE.to_string();
            return HandleText::Command(MatrixCommand::new_from_client(rw, set_state));
        }
        if let Ok(set_visibility) = serde_json::from_str::<SetVisibility>(&text){
            let channel =set_visibility.channel.parse::<i32>();
            let value = set_visibility.value.parse::<bool>();
            if let Err(e) = channel {
                return HandleText::Error(e.to_string())
            }
            if let Err(e) = value{
                return HandleText::Error(e.to_string())
            }
            return HandleText::SetVisibility(set_visibility);
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

impl WsSession {

    pub async fn attach_channel_visibility(
        states: &mut MatrixStates,
        socket_id: i32,
        pgpool: web::Data<AppState>,
    ) -> Result<MatrixStates,()> {
        let pgpool = pgpool;
        let mut i_visibility_map: HashMap<u32, bool> = HashMap::new();
        let mut o_visibility_map: HashMap<u32, bool> = HashMap::new();
        let i_channels = interfaces::retrieve_channels(&pgpool, socket_id, SRC::INPUT)
            .await
            .unwrap();
            
        let o_channels = interfaces::retrieve_channels(&pgpool, socket_id, SRC::OUTPUT)
            .await
            .unwrap();
        if i_channels.is_none() || o_channels.is_none(){
            return Err(());
        }
        i_channels.unwrap().iter().enumerate().for_each(|(i, channel)| {
            let index = i + 1;
            i_visibility_map
                .entry(index as u32)
                .or_insert(channel.visible);
        });
        o_channels.unwrap().iter().enumerate().for_each(|(i, channel)| {
            let index = i + 1;
            o_visibility_map
                .entry(index as u32)
                .or_insert(channel.visible);
        });

        states.i_visibility = Some(i_visibility_map);
        states.o_visibility = Some(o_visibility_map);
        Ok(states.clone())
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}
