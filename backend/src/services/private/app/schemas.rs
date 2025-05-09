use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engines::{audio_engine::lib::MatrixCommand, video_engine::defs::CameraCommand};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetState {
    pub section: String,
    pub io: Option<String>,
    pub channel: Option<String>,
    pub value: Option<String>,
    pub index: Option<String>,
    pub velocity: Option<String>,
    pub direction: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetAttributes {
    pub io: Option<String>,
    pub channel: Option<String>,
    pub index: Option<String>,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct StreamError {
    pub fail_reason: String,
    pub at_socket: String,
}

#[derive(Deserialize)]
pub struct SessionUUID {
    pub uuid: String,
}

#[derive(Debug, Clone)]

pub enum DeviceCommnd{
    MatrixCommand(MatrixCommand),
    CameraCommand(CameraCommand)
}

pub fn index_values<T>(indexable: Vec<T>,from_0:bool) -> HashMap<u32, T>
where
    T: std::fmt::Display,
{
    let mut map: HashMap<u32, T> = HashMap::new();
    indexable
        .into_iter()
        .enumerate()
        .for_each(|(i, indexable_unit)| {
            let index ;
            if from_0 {index= i} else{index = i+1}
            map.insert(index as u32, indexable_unit);
        });
    return map;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatrixStates {
    pub i_mute: HashMap<u32, bool>,
    pub o_mute: HashMap<u32, bool>,
    pub i_volumes: HashMap<u32, f32>,
    pub o_volumes: HashMap<u32, f32>,
    pub i_visibility: HashMap<u32, bool>,
    pub o_visibility: HashMap<u32, bool>,
    pub i_labels: HashMap<u32, String>,
    pub o_labels: HashMap<u32, String>,
    pub mix_map: HashMap<String,bool>,
    pub preset_labels:HashMap<u32, String>,
    pub current_preset: u8,
    pub available: Option<bool>,
    pub device_type: String,
    pub matrix_socket: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CameraStates{
    pub preset_labels: HashMap<u32,String>,
    pub available: Option<bool>,
    pub device_type: String,
    pub camera_socket: String,
    pub current_preset: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub enum MachineStates {
    MatrixStates(MatrixStates),
    CameraStates(CameraStates),
}
