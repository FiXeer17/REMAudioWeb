use crate::utils::configs::DatabaseEnv;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebClaims {
    pub sub: i32,
    pub session_type: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NativeClaims {
    pub sub: i32,
    pub session_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Claims {
    Web(WebClaims),
    Native(NativeClaims),
}

pub fn id_to_jwt(id: i32, session_type: String) -> Result<String, Box<dyn std::error::Error>> {
    let jwt_secret = DatabaseEnv::get_jwt_secret();
    let claims: Claims;

    if session_type == "web" {
        let default_jwt_duration: i64 = 3; // hours

        let jwt_exp = Utc::now()
            .checked_add_signed(chrono::Duration::hours(default_jwt_duration))
            .unwrap()
            .timestamp();

        claims = Claims::Web(WebClaims {
            session_type,
            sub: id,
            exp: jwt_exp as usize,
        });
    } else {
        claims = Claims::Native(NativeClaims {
            sub: id,
            session_type,
        });
    }
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?)
}

pub fn jwt_to_id(jwt_token: String) -> Result<i32, jsonwebtoken::errors::Error> {
    let jwt_secret = DatabaseEnv::get_jwt_secret();
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.required_spec_claims.remove("exp");
    let claim = decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;
    match claim.claims {
        Claims::Native(c) => Ok(c.sub),
        Claims::Web(c) => {
            let now = chrono::Utc::now().timestamp() as usize;
            if now > c.exp {
                return Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature,
                ));
            }
            return Ok(c.sub);
        }
    }
}


pub fn bearertkn_to_id(bearer_token:&str) -> i32{
    let prefix = "Bearer ";
    let jwt_token = bearer_token.strip_prefix(prefix).unwrap();
    jwt_to_id(jwt_token.to_string()).unwrap()
}