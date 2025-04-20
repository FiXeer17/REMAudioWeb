
use crate::{
    engine::{defs::errors::Error, lib::MatrixCommand},
    services::{private::app::schemas::SetAttributes, public::utils::SRC},
    utils::configs::channels_settings,
};

#[derive(Debug, Clone)]
pub enum HandleText {
    Command(Result<MatrixCommand, Error>),
    SetVisibility(SetAttributes),
    SetLabels(SetAttributes),
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
