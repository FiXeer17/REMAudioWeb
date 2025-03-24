use crate::utils::common::validate_session_type;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUser {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(custom = "validate_session_type")]
    pub session_type: String,
}

#[derive(Serialize, Debug, FromRow)]
pub struct ReturnCreateUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}
#[derive(Serialize, Debug)]
pub struct ReturnCreateUserJWT {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub access_token: String,
}
