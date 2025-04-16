use actix_web::web::Data;

use crate::{
    engine::{defs::errors::Error, lib::MatrixCommand},
    services::{private::app::schemas::SetVisibility, public::utils::SRC},
    utils::configs::channels_settings,
    AppState,
};

#[derive(Debug, Clone)]
pub enum HandleText {
    Command(Result<MatrixCommand, Error>),
    SetVisibility(SetVisibility),
    Recache,
    Error(String),
}

#[derive(Clone)]
pub struct UpdateVisibility {
    pub db: Data<AppState>,
    pub user_id: i32,
    pub set_visibility: SetVisibility,
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
