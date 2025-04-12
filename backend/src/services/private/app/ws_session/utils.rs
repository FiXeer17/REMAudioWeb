
use actix_web::web::Data;

use crate::{engine::{defs::errors::Error, lib::MatrixCommand}, services::private::app::schemas::SetVisibility, AppState};


#[derive(Debug,Clone)]
pub enum HandleText{
    Command(Result<MatrixCommand, Error>),
    SetVisibility(SetVisibility),
    Recache,
    Error(String),
}

#[derive(Clone)]
pub struct UpdateVisibility{
    pub db : Data<AppState>,
    pub user_id:i32,
    pub set_visibility: SetVisibility
}