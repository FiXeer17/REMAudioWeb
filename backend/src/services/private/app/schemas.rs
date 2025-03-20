use crate::engine::defs::{datas::io::SRC, fncodes::FNCODE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engine::lib::{MatrixCommand, MatrixCommandDatas};

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


#[derive(Serialize, Deserialize, Clone)]
pub struct StreamError {
    pub fail_reason: String,
    pub at_socket: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatrixStates {
    pub i_mute: HashMap<u32, bool>,
    pub o_mute: HashMap<u32, bool>,
    pub i_volumes: HashMap<u32, f32>,
    pub o_volumes: HashMap<u32, f32>,
    pub current_preset: u8,
}
impl MatrixStates {
    pub fn new(cmds: Vec<MatrixCommand>) -> Self {


        let (mut i_mute, mut o_mute, mut i_volumes, mut o_volumes, mut current_preset): (
            HashMap<u32, bool>,
            HashMap<u32, bool>,
            HashMap<u32, f32>,
            HashMap<u32, f32>,
            u8,
        ) = (
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            0,
        );

        let input_from_def = SRC::INPUT.to_label();

        for command in cmds {
            let cmd = MatrixCommandDatas::from(command);
            let io = cmd.io;
            let function = cmd.function;
            
            if function == FNCODE::MUTE.to_label(){
                let (muted,io,channel) = (cmd.muted.unwrap(),io.unwrap(),cmd.channel.unwrap());
                if io == input_from_def{
                    i_mute.entry(channel).or_insert(muted);
                    continue;
                }
                o_mute.entry(channel).or_insert(muted);
            }else if function == FNCODE::VOLUME.to_label(){
                let (value,io,channel) = (cmd.value.unwrap(),io.unwrap(),cmd.channel.unwrap()); 
                if io == input_from_def{
                    i_volumes.entry(channel).or_insert(value);
                    continue;
                }
                o_volumes.entry(channel).or_insert(value);
            }else{
                current_preset = cmd.preset.unwrap();
            }
            
        
        }

        Self {
            i_mute,
            o_mute,
            i_volumes,
            o_volumes,
            current_preset,
        }
    }

    pub fn set_changes(&mut self,command:MatrixCommand) -> () {
        let cmd = MatrixCommandDatas::from(command);
        let io = cmd.io;
        let function = cmd.function;
        let input_from_def = SRC::INPUT.to_label();

        
        if function == FNCODE::MUTE.to_label(){
            let (muted,io,channel) = (cmd.muted.unwrap(),io.unwrap(),cmd.channel.unwrap());
            if io == input_from_def{
                self.i_mute.entry(channel).or_insert(muted);
            }
            self.o_mute.entry(channel).or_insert(muted);
        }else if function == FNCODE::VOLUME.to_label(){
            let (value,io,channel) = (cmd.value.unwrap(),io.unwrap(),cmd.channel.unwrap()); 
            if io == input_from_def{
                self.i_volumes.entry(channel).or_insert(value);
            }
            self.o_volumes.entry(channel).or_insert(value);
        }else{
            self.current_preset = cmd.preset.unwrap();
        }
        
    }

    
}
