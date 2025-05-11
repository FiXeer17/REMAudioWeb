use super::utils::HandleText;
use crate::{
    configs::websocket_settings, engines::{audio_engine::{
        defs::datas,
        lib::MatrixCommand,
    }, sections::Sections, video_engine::{camera_presets_lib::call_preset, defs::{fncodes::FNCODE, CameraCommand}, tilt_pan_lib::move_camera, zoom_lib::{zoom_tele, zoom_wide}}}, services::private::{app::{
        messages::{self, Disconnect},
        schemas::{SetAttributes, SetState},
    }, socket::utils::Device}, AppState
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
                    Sections::CameraCommand(_) => return handle_video_command(set_state,&section),
                    Sections::Visibility => return handle_visibility(set_state),
                    Sections::ChannelLabels => return handle_channel_label(set_state),
                    Sections::MatrixPresetLabels => return handle_preset_label(set_state),
                    Sections::CameraPresetLabels => return handle_preset_label(set_state)
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
    let Ok(_) = ch.parse::<u32>()else{
        return HandleText::Error("channel is invalid".to_string());
    };
    let Ok(_) = value.parse::<bool>() else{
        return HandleText::Error("value is invalid".to_string());
    };

    let set_visibility = SetAttributes {
        io: set_state.io,
        channel: set_state.channel,
        value: set_state.value.unwrap(),
        index:None,
        device:None
    };
    HandleText::SetVisibility(set_visibility)
}

//fn handle_label(set_state: SetState) -> HandleText{}
fn handle_matrix_command(set_state: SetState) -> HandleText {
    let rw = datas::rw::WRITE.to_string();
    HandleText::MatrixCommand(MatrixCommand::new_from_client(rw, set_state))
}
fn handle_video_command(set_state: SetState,section:&Sections) -> HandleText{
    match section{
        Sections::CameraCommand(sc) =>match sc{
            FNCODE::Preset => {
                let call_preset  = call_preset(set_state.value.unwrap());
                if let Err(e) = call_preset {
                    return HandleText::CameraCommand(Err(e));
                }else{
                    return HandleText::CameraCommand(Ok(CameraCommand{fncode:sc.clone(),cmd:call_preset.unwrap()}));
                }
            },                
            FNCODE::ZoomTele => {
                let zoom_tele  = zoom_tele(set_state.value.unwrap());
                if let Err(e) = zoom_tele {
                    return HandleText::CameraCommand(Err(e));
                }else{
                    return HandleText::CameraCommand(Ok(CameraCommand{fncode:sc.clone(),cmd:zoom_tele.unwrap()}));
                }
            },
            FNCODE::ZoomWide => {
                let zoom_wide  = zoom_wide(set_state.value.unwrap());
                if let Err(e) = zoom_wide {
                    return HandleText::CameraCommand(Err(e));
                }else{
                    return HandleText::CameraCommand(Ok(CameraCommand{fncode:sc.clone(),cmd:zoom_wide.unwrap()}));
                }
            },
            FNCODE::MoveCamera =>{
                let Some(velocity) = set_state.velocity else {return HandleText::Error("Velocity not found".to_string())};
                let Some(direction) = set_state.direction else {return HandleText::Error("Direction not found".to_string())};
                let move_camera = move_camera(velocity, direction);
                if let Err(e) = move_camera{
                    return HandleText::CameraCommand(Err(e));
                }
                HandleText::CameraCommand(Ok(CameraCommand { fncode: sc.clone(), cmd: move_camera.unwrap() }))
            }
        },
        _ => HandleText::Error("Invalid video command".to_string())
    }
}

fn handle_channel_label(set_state: SetState) -> HandleText{
    let Some(_) = set_state.io else{
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
    HandleText::SetChannelLabels(SetAttributes{io:set_state.io,channel:set_state.channel,value,index:None,device:None})
}

fn handle_preset_label(set_state: SetState) -> HandleText{
    let Some(index) = set_state.index.clone() else {
        return HandleText::Error("index is none".to_string());
    };
    let Some(value) = set_state.value.clone() else {
        return HandleText::Error("value is none".to_string());
    };
    let Ok(_) = index.parse::<u32>() else{
        return HandleText::Error("cannot parse index".to_string());
    };
    let section = Sections::from_str(&set_state.section).unwrap();
    match section{
        Sections::MatrixPresetLabels =>HandleText::SetPresetLabels(SetAttributes{io:None,index:set_state.index,value,channel:None,device:Some(Device::Audio)}),
        Sections::CameraPresetLabels =>HandleText::SetPresetLabels(SetAttributes{io:None,index:set_state.index,value,channel:None,device:Some(Device::Video)}),
        _ => HandleText::Error("Invalid section".to_string())
    }
}      

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.on_connect(ctx);
    }
}
