use crate::engine::defs::{datas::io::SRC, fncodes::FNCODE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engine::lib::{MatrixCommand, MatrixCommandDatas};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetState {
    pub section: String,
    pub io: Option<String>,                   
    pub channel: Option<String>,
    pub value : Option<String>
}



#[derive(Serialize, Deserialize, Clone)]
pub struct StreamError {
    pub fail_reason: String,
    pub at_socket: String,
}

#[derive(Deserialize)]
pub struct SessionUUID{
    pub uuid: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatrixStates {
    pub i_mute: HashMap<u32, bool>,
    pub o_mute: HashMap<u32, bool>,
    pub i_volumes: HashMap<u32, f32>,
    pub o_volumes: HashMap<u32, f32>,
    pub current_preset: u8,
    pub available: Option<bool>,
    pub matrix_socket: String,
}
impl MatrixStates {
    pub fn new(cmds: Vec<MatrixCommand>,matrix_socket:String) -> Self {


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
            available:None,
            matrix_socket,
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
                self.i_mute.insert(channel,muted);
                return;
            }
            self.o_mute.insert(channel,muted);
        }else if function == FNCODE::VOLUME.to_label(){
            let (value,io,channel) = (cmd.value.unwrap(),io.unwrap(),cmd.channel.unwrap()); 
            if io == input_from_def{
                self.i_volumes.insert(channel,value);
                return;
            }
            self.o_volumes.insert(channel,value);
        }else{
            self.current_preset = cmd.preset.unwrap();
        }
        
    }
    
}
