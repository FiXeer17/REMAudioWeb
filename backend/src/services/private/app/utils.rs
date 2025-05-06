
use std::collections::HashMap;


use crate::{audio_engine::{defs::{datas::io::SRC, fncodes::FNCODE}, lib::{MatrixCommand, MatrixCommandDatas}}, services::private::socket::utils::Device};

use super::{messages::Commands, schemas::{index_values, CameraStates, MachineStates, MatrixStates}};



pub trait CommandsExt {
    fn compatibility(&self)->Device;
}

impl CommandsExt for Commands{
    fn compatibility(&self)->Device {
        match self {
            Commands::ReCache => Device::Audio,
            Commands::SetChannelLabel(_) => Device::Audio,
            Commands::SetPresetLabel(_) => Device::Audio,
            Commands::SetMatrixCommand(_)=> Device::Audio,
            Commands::SetVisibility(_)=> Device::Audio,
            
        }
    }
}


impl MatrixStates {
    pub fn new(
        cmds: Vec<MatrixCommand>,
        matrix_socket: String,
        input_channel_labels: Vec<String>,
        output_channel_labels: Vec<String>,
        preset_labels: Vec<String>,
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
        let i_labels: HashMap<u32, String> = index_values(input_channel_labels);
        let o_labels: HashMap<u32, String> = index_values(output_channel_labels);
        let preset_labels: HashMap<u32,String> = index_values(preset_labels);
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
            preset_labels,
            current_preset,
            available: None,
            matrix_socket,
            device_type: Device::Audio.to_string()
        }
    }

    pub fn set_changes(&mut self, command: MatrixCommand) {
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

pub trait HasPresetLabels: Send + 'static {
    fn preset_labels_mut(&mut self) -> &mut HashMap<u32,String>;
}

impl HasPresetLabels for MatrixStates{
    fn preset_labels_mut(&mut self) -> &mut HashMap<u32,String> {
        &mut self.preset_labels
    }
}

impl HasPresetLabels for CameraStates{
    fn preset_labels_mut(&mut self) -> &mut HashMap<u32,String> {
        &mut self.preset_labels
    }
}

impl MachineStates{
    pub fn as_mut_trait(&mut self) -> Option<&mut dyn HasPresetLabels>{
        match self{
            Self::CameraStates(cs) => Some(cs),
            Self::MatrixStates(ms) => Some(ms)
        }
    }
}

impl CameraStates{
    pub fn new(
        camera_socket: String,
        preset_labels: Vec<String>
    ) -> Self{
        let preset_labels: HashMap<u32,String> = index_values(preset_labels);
        Self {
            preset_labels,
            avaiable:None,
            device_type: Device::Video.to_string(),
            camera_socket
        }
    }
}