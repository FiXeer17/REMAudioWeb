use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Channel {
    pub id: i32,
    pub channel_name: String,
    pub visible: bool,
    pub socket_id: i32,
}
#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Preset {
    pub id:i32,
    pub label:String,
    pub relative_identifier:i32,
    pub socket_id:i32,
}

pub trait IsContainedExt{
    fn socket_is_contained(&self,socket:&str)->bool;
    fn latest_is_contained(&self)-> Option<Socket>;
}
impl IsContainedExt for HashSet<Socket>{
    fn socket_is_contained(&self,socket:&str)->bool {
        self.iter().any(|s|&s.device == socket)
    }
    fn latest_is_contained(&self) -> Option<Socket>{
        self.iter().find_map(|s|{if s.latest {return Some(s.clone())}; None})
    }
}

#[derive(Deserialize, Serialize, Clone,Debug, FromRow,Eq)]
pub struct Socket {
    pub id: Option<i32>,
    pub socket_name: String,
    pub socket: String,
    pub latest: bool,
    pub device: String,
    pub latest_preset: Option<i32>
}

impl Hash for Socket{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.socket.hash(state);
    }
}

impl PartialEq<Socket> for Socket{
    fn eq(&self, other: &Self) -> bool {
        self.socket == other.socket
    }
    fn ne(&self, other: &Self) -> bool {
        self.socket != other.socket
    }
}



