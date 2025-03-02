use crate::utils::env_dns::Env;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub session_type: String,
    pub exp: usize,
}

pub fn id_to_jwt(id: i32, session_type: String) -> Result<String, Box<dyn std::error::Error>> {
    let default_jwt_duration: i64 = 3; // hours

    let jwt_exp = Utc::now()
        .checked_add_signed(chrono::Duration::hours(default_jwt_duration))
        .unwrap()
        .timestamp();

    let jwt_secret = Env::get_jwt_secret();

    let claims = Claims {
        session_type,
        sub: id,
        exp: jwt_exp as usize,
    };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn jwt_to_id(jwt_token: String) -> Result<i32, jsonwebtoken::errors::Error> {
    let jwt_secret = Env::get_jwt_secret();
    let claim = decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(claim.claims.sub)
}
