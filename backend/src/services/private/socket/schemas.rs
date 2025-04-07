use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct SetSocketBody{
    pub socket: String
}