use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct B64Address{
    pub a:String
}
