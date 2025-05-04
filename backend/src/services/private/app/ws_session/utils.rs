
use crate::{
    audio_engine::{defs::errors::Error, lib::MatrixCommand}, configs::{channels_settings, presets_settings}, services::{private::{app::schemas::SetAttributes, socket::utils::Device}, public::utils::SRC}
};

#[derive(Debug, Clone)]
pub enum HandleText {
    Command(Result<MatrixCommand, Error>),
    SetVisibility(SetAttributes),
    SetChannelLabels(SetAttributes),
    SetPresetLabels(SetAttributes),
    Recache,
    Error(String),
}


pub fn check_channel(io:String,ch: u8) -> bool {
    if io == SRC::INPUT.to_string() {
        if ch <= channels_settings::get_i_channel_number() && ch > 0 {
            return true;
        }
    } else if io == SRC::OUTPUT.to_string() {
        if ch <= channels_settings::get_o_channel_number() && ch > 0 {
            return true;
        }
    }
    return false;
}

pub fn check_preset(preset:u8,device:Device) ->bool{
    match device{
        Device::Audio => {if preset <= presets_settings::get_audio_presets_number() && preset >0 {return true;}},
        Device::Video =>{if preset <= presets_settings::get_video_presets_number() && preset >0 {return true;}}
    }
    return false;
}
