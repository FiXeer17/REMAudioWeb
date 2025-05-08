
use std::{collections::HashMap, net::SocketAddrV4, str::FromStr};


use crate::{audio_engine::{defs::{datas::io::SRC, fncodes::FNCODE}, lib::{MatrixCommand, MatrixCommandDatas}}, services::private::socket::utils::Device};

use super::{messages::{CameraReady, Commands, DeviceReady, MatrixReady}, schemas::{index_values, CameraStates, MachineStates, MatrixStates}};



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
        let (mut i_mute, mut o_mute, mut i_volumes, mut o_volumes,mut mix_map, mut current_preset): (
            HashMap<u32, bool>,
            HashMap<u32, bool>,
            HashMap<u32, f32>,
            HashMap<u32, f32>,
            HashMap<String,bool>,
            u8,
            
        ) = (
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            0,
        );

        let i_labels: HashMap<u32, String> = index_values(input_channel_labels);
        let o_labels: HashMap<u32, String> = index_values(output_channel_labels);
        let preset_labels: HashMap<u32,String> = index_values(preset_labels);
        let i_visibility: HashMap<u32, bool> = index_values(input_visibility);
        let o_visibility: HashMap<u32, bool> = index_values(output_visibility);
        

        for command in cmds {
            let cmd = MatrixCommandDatas::from(command);
            let function = cmd.function.clone();
            match FNCODE::from_str(&function){
                Ok(FNCODE::MUTE) => Self::handle_mute_cmd(&mut i_mute, &mut o_mute, &cmd),
                Ok(FNCODE::VOLUME) => Self::handle_volume_cmd(&mut i_volumes, &mut o_volumes, &cmd),
                Ok(FNCODE::SCENE) => Self::handle_preset_cmd(&mut current_preset, &cmd),
                Ok(FNCODE::MATRIXMIXING) => Self::handle_mix_cmd(&mut mix_map,&cmd),
                _ => panic!("invalid cmd")
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
            mix_map,
            device_type: Device::Audio.to_string()
        }
    }

    pub fn set_changes(&mut self, command: MatrixCommand) {
        let cmd = MatrixCommandDatas::from(command);
        let function = cmd.function.clone();
        match FNCODE::from_str(&function){
            Ok(FNCODE::MUTE) => Self::handle_mute_cmd(&mut self.i_mute, &mut self.o_mute, &cmd),
            Ok(FNCODE::VOLUME) => Self::handle_volume_cmd(&mut self.i_volumes, &mut self.o_volumes, &cmd),
            Ok(FNCODE::SCENE) => Self::handle_preset_cmd(&mut self.current_preset, &cmd),
            Ok(FNCODE::MATRIXMIXING) => Self::handle_mix_cmd(&mut self.mix_map,&cmd),
            _ => panic!("invalid cmd")
        }
    }

    fn handle_mute_cmd(i_mute:&mut HashMap<u32, bool>, o_mute:&mut HashMap<u32, bool>,cmd:&MatrixCommandDatas){
        let (muted, io, channel) = (cmd.muted.unwrap(), cmd.io.clone().unwrap(), cmd.channel.unwrap());

        if io == SRC::INPUT.to_label() {
            i_mute.entry(channel).or_insert(muted);
            return;
        }
        o_mute.entry(channel).or_insert(muted);
    }

    fn handle_volume_cmd(i_volumes: &mut HashMap<u32, f32>,o_volumes: &mut HashMap<u32, f32>, cmd:&MatrixCommandDatas ){
        let (value, io, channel) = (cmd.value.unwrap(), cmd.io.clone().unwrap(), cmd.channel.unwrap());
        if io == SRC::INPUT.to_label() {
            i_volumes.insert(channel, value);
            return;
        }
        o_volumes.insert(channel, value);
    }
    fn handle_preset_cmd(current_preset:&mut u8, cmd:&MatrixCommandDatas){
        let preset = cmd.preset.unwrap();
        *current_preset = preset;
    }
    fn handle_mix_cmd(mix_map: &mut HashMap<String,bool>,cmd:&MatrixCommandDatas){
        let indx = cmd.index.unwrap();
        let ch = cmd.channel.unwrap();
        let connected = cmd.connected.unwrap();
        mix_map.insert(format!("({},{})",indx,ch),connected);
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
pub trait DeviceState{
    fn get_available(&self) -> Option<bool>;
    fn set_avaiable(&mut self,available:bool);
}
pub trait HasStatesMessage {
    fn get_states(&self) -> MachineStates;
    fn get_socket(&self) -> SocketAddrV4;
}

impl CameraStates{
    pub fn new(
        camera_socket: String,
        preset_labels: Vec<String>,
        current_preset: i32
    ) -> Self{
        let preset_labels: HashMap<u32,String> = index_values(preset_labels);
        Self {
            preset_labels,
            available:None,
            device_type: Device::Video.to_string(),
            camera_socket,
            current_preset,
        }
    }
}


impl HasStatesMessage for CameraReady{
    fn get_states(&self) -> MachineStates {
        MachineStates::CameraStates(self.states.clone())
    }
    fn get_socket(&self) -> SocketAddrV4 {
        self.socket
    }
}
impl HasStatesMessage for MatrixReady{
    fn get_states(&self) -> MachineStates {
        MachineStates::MatrixStates(self.states.clone())
    }
    fn get_socket(&self) -> SocketAddrV4 {
        self.socket
    }
}
impl HasStatesMessage for DeviceReady{
    fn get_states(&self) -> MachineStates {
        match self{
            Self::CameraReady(cr) => MachineStates::CameraStates(cr.states.clone()),
            Self::MatrixReady(mr) => MachineStates::MatrixStates(mr.states.clone())
        }
    }
    fn get_socket(&self) -> SocketAddrV4 {
        match self{
            Self::CameraReady(cr) => cr.socket,
            Self::MatrixReady(mr) => mr.socket
        }
    }
}

impl DeviceState for CameraStates{
    fn get_available(&self) -> Option<bool> {
        self.available
    }
    fn set_avaiable(&mut self,available:bool) {
        self.available = Some(available);
    }
}

impl DeviceState for MatrixStates{
    fn get_available(&self) -> Option<bool> {
        self.available
    }
    fn set_avaiable(&mut self,available:bool) {
        self.available = Some(available);
    }
}

impl DeviceState for MachineStates{
    fn get_available(&self) -> Option<bool> {
        match self{
            MachineStates::CameraStates(cs) => cs.available,
            MachineStates::MatrixStates(ms) => ms.available
        }
    }
    fn set_avaiable(&mut self,available:bool) {
        match self{
            MachineStates::CameraStates(cs)=> cs.set_avaiable(available),
            MachineStates::MatrixStates(ms)=> ms.set_avaiable(available),
        }
    }
}
