use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engine::lib::MatrixCommandDatas;



#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct ResponseAllStatesValue {
    pub datas: Vec<State>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct State {
    pub side: String,                      // audio/video
    pub section: String,                   // volume/presets/mute
    pub values: HashMap<String, Vec<i32>>, // value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestSetMute {
    pub section: String,
    pub action: String,
    pub channel: String,
    pub src: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestSetGain {
    pub section: String,
    pub action: String,
    pub channel: String,
    pub src: String,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestSetPreset {
    pub section: String,
    pub action: String,
    pub preset: i32,
}

pub enum WsRequests {
    SetMute(RequestSetMute),
    SetGain(RequestSetGain),
    SetPreset(RequestSetPreset),
}

#[derive(Serialize,Deserialize)]
pub struct ReturnReadAll{
    datas: Vec<MatrixCommandDatas>
}