use crate::utils::common::validate_session_type;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, FromRow)]

pub struct ReturnFullUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SignIn {
    pub username: String,
    pub password: String,
    #[validate(custom = "validate_session_type")]
    pub session_type: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SignInReturn {
    pub access_token: String,
    pub admin: bool
}
