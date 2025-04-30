use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSocketBody {
    pub socket_name: String,
    pub socket: String,
    pub device_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveSocketBody {
    pub socket: String,
}
