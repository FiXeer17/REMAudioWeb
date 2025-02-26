use crate::utils::env_dns::Env;
use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub session_type: String,
    pub exp: usize,
}

pub fn argon2_enc(password: &str) -> Result<String, Error> {
    // create the salt
    let salt = SaltString::generate(&mut OsRng);
    // init the default argon parameters
    let argon2 = Argon2::default();
    // hash the password and return
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn argon2_verify(hash: &str, password: &str) -> Result<bool, Error> {
    // parsing the phc hash into PasswordHash type
    let hash = PasswordHash::new(hash)?;

    // verifying if the password produce the same output and return it
    match Argon2::default().verify_password(password.as_bytes(), &hash) {
        Ok(_) => return Ok(true),                 // if is equal return true
        Err(Error::Password) => return Ok(false), // if is not equal return false
        Err(e) => return Err(e),                  // if it can't verify propagate the error
    }
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
