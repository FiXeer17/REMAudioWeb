use crate::{
    configs::{channels_settings, presets_settings}, engines::{audio_engine::lib::MatrixCommand, video_engine::defs::CameraCommand}, services::private::{app::schemas::SetAttributes, socket::utils::Device}
};
use crate::engines::{audio_engine,video_engine};


#[derive(Debug, Clone)]
pub enum HandleText {
    MatrixCommand(Result<MatrixCommand, audio_engine::defs::errors::Error>),
    CameraCommand(Result<CameraCommand,video_engine::defs::errors::Error>),
    SetVisibility(SetAttributes),
    SetChannelLabels(SetAttributes),
    SetPresetLabels(SetAttributes),
    Recache,
    Error(String),
}

pub fn check_channel(ch: u8) -> bool {
    if ch <= channels_settings::get_channels_number() && ch > 0 {
        return true;
    }
    return false;
}

pub fn check_preset(preset: u8, device: Device) -> bool {
    match device {
        Device::Audio => {
            if preset <= presets_settings::get_audio_presets_number() && preset > 0 {
                return true;
            }
        }
        Device::Video => {
            if preset <= presets_settings::get_video_presets_number() && preset > 0 {
                return true;
            }
        }
    }
    return false;
}
