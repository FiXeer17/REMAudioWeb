use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use validator::{Validate,ValidationError};


fn validate_session_type(session_type: &str) -> Result<(), ValidationError> {
    match session_type {
        "web" | "native" => Ok(()),
        _ => Err(ValidationError::new("invalid_session_type")),
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUser {
    #[validate(length(min = 4, max = 254))]
    pub username: String,
    #[validate(length(min = 4, max = 254), email)]
    pub email: String,
    pub password: String,
    #[validate(custom="validate_session_type")]
    pub session_type: String
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
    pub jwt_token: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]

pub struct ReturnFullUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    
}

#[derive(Deserialize,Serialize, Debug, Validate)]
pub struct SignIn{
    #[validate(length(min = 4, max = 254), email)]
    pub email: String,
    pub password: String,
    #[validate(custom="validate_session_type")]
    pub session_type: String
}