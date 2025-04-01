use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};

use serde_json::{json, Value};
use validator::ValidationError;

pub fn validate_session_type(session_type: &str) -> Result<(), ValidationError> {
    match session_type {
        "web" | "native" => Ok(()),
        _ => Err(ValidationError::new("invalid_session_type")),
    }
}

pub fn return_json_reason(reason: &str) -> Value {
    json!({"reason":reason})
}

pub fn check_socket(sock: String) -> Result<Option<SocketAddrV4>,std::io::Error>{
    let converted  = sock.to_socket_addrs();
    if let Err(e) = converted{
        return Err(e);
    };
    Ok(converted.unwrap().find_map(|sock| {
        if let SocketAddr::V4(sockv4) = sock {
            Some(sockv4)
        } else {
            None
        }
    }))
}