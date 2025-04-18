use crate::engine::defs::{datas::io::SRC, fncodes::FNCODE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engine::lib::{MatrixCommand, MatrixCommandDatas};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetState {
    pub section: String,
    pub io: Option<String>,
    pub channel: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetVisibility {
    pub io: String,
    pub channel: String,
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

pub fn index_values<T>(indexable: Vec<T>) -> HashMap<u32, T>
where
    T: std::fmt::Display,
{
    let mut map: HashMap<u32, T> = HashMap::new();
    indexable
        .into_iter()
        .enumerate()
        .for_each(|(i, indexable_unit)| {
            let index = i + 1;
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
    pub current_preset: u8,
    pub available: Option<bool>,
    pub matrix_socket: String,
}
impl MatrixStates {
    pub fn new(
        cmds: Vec<MatrixCommand>,
        matrix_socket: String,
        input_labels: Vec<String>,
        output_labels: Vec<String>,
        input_visibility: Vec<bool>,
        output_visibility: Vec<bool>,
    ) -> Self {
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
        let i_labels: HashMap<u32, String> = index_values(input_labels);
        let o_labels: HashMap<u32, String> = index_values(output_labels);
        let i_visibility: HashMap<u32, bool> = index_values(input_visibility);
        let o_visibility: HashMap<u32, bool> = index_values(output_visibility);

        for command in cmds {
            let cmd = MatrixCommandDatas::from(command);
            let io = cmd.io;
            let function = cmd.function;

            if function == FNCODE::MUTE.to_label() {
                let (muted, io, channel) = (cmd.muted.unwrap(), io.unwrap(), cmd.channel.unwrap());
                if io == input_from_def {
                    i_mute.entry(channel).or_insert(muted);
                    continue;
                }
                o_mute.entry(channel).or_insert(muted);
            } else if function == FNCODE::VOLUME.to_label() {
                let (value, io, channel) = (cmd.value.unwrap(), io.unwrap(), cmd.channel.unwrap());
                if io == input_from_def {
                    i_volumes.entry(channel).or_insert(value);
                    continue;
                }
                o_volumes.entry(channel).or_insert(value);
            } else {
                current_preset = cmd.preset.unwrap();
            }
        }

        Self {
            i_mute,
            o_mute,
            i_volumes,
            o_volumes,
            i_visibility,
            o_visibility,
            i_labels,
            o_labels,
            current_preset,
            available: None,
            matrix_socket,
        }
    }

    pub fn set_changes(&mut self, command: MatrixCommand) -> () {
        let cmd = MatrixCommandDatas::from(command);
        let io = cmd.io;
        let function = cmd.function;
        let input_from_def = SRC::INPUT.to_label();

        if function == FNCODE::MUTE.to_label() {
            let (muted, io, channel) = (cmd.muted.unwrap(), io.unwrap(), cmd.channel.unwrap());
            if io == input_from_def {
                self.i_mute.insert(channel, muted);
                return;
            }
            self.o_mute.insert(channel, muted);
        } else if function == FNCODE::VOLUME.to_label() {
            let (value, io, channel) = (cmd.value.unwrap(), io.unwrap(), cmd.channel.unwrap());
            if io == input_from_def {
                self.i_volumes.insert(channel, value);
                return;
            }
            self.o_volumes.insert(channel, value);
        } else {
            self.current_preset = cmd.preset.unwrap();
        }
    }
}
