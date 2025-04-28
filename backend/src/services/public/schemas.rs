use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Channel {
    pub id: i32,
    pub channel_name: String,
    pub visible: bool,
    pub socket_id: i32,
}

#[derive(Deserialize, Serialize, Clone,Debug, FromRow)]
pub struct Socket {
    pub id: Option<i32>,
    pub socket_name: String,
    pub socket: String,
    pub latest: bool,
}


impl PartialEq for Socket{
    fn eq(&self, other: &Self) -> bool {
        self.socket == other.socket
    }
    fn ne(&self, other: &Self) -> bool {
        self.socket != other.socket
    }
}

