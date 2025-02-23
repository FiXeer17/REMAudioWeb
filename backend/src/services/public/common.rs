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
