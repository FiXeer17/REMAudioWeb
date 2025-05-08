
use std::str::FromStr;

use super::{
    defs::{datas::{matrix_mixing_status::MatrixMixingStatus, rw::READ}, errors::Error, fncodes::FNCODE},
    lib::MatrixCommand,
};
use crate::{configs::channels_settings, services::private::app::schemas::SetState};

pub fn read_mix_ch(indx:u32,ch: u32) -> Result<MatrixCommand, Error>{
    let (ch,indx) = (format!("{:02X}", ch),format!("{:02X}", indx));
    let _ =MatrixCommand::check_channel(&ch)?;
    let _ =MatrixCommand::check_channel(&indx)?;
    let data = vec![indx,ch];
    let rw = READ.to_string();
    let fcode = FNCODE::MATRIXMIXING.to_string();
    MatrixCommand::new(rw,fcode,Some(data))
}
pub fn read_mix_all() -> Result<Vec<MatrixCommand>, Error> {
    let mut commands:Vec<MatrixCommand> = Vec::new();
    for indx in 1..=channels_settings::get_channels_number(){
        for ch in 1..=channels_settings::get_channels_number(){
            commands.push(read_mix_ch(indx as u32, ch as u32)?)
        }
    }
    Ok(commands)
}

pub fn into_data(data: SetState) -> Result<Vec<String>, Error> {
    let index = format!(
        "{:02X}",
        data.index.unwrap().trim().parse::<u8>().unwrap()
    );
    let channel = format!(
        "{:02X}",
        data.channel.unwrap().trim().parse::<u8>().unwrap()
    );
    MatrixCommand::check_channel(&channel)?;
    MatrixCommand::check_channel(&index)?;
    
    let value = MatrixMixingStatus::from_str(&data.value.unwrap())?;
    
    Ok(vec![index, channel, value.to_string()])
}


pub fn into_deserialized(mut data:Vec<String>) -> (Option<u32>,Option<u32>,Option<bool>){
    let mut connected = None;
    let indx = Some(
        u32::from_str_radix(&data.remove(0), 16).expect("Cannot convert index"),
    );
    let ch = Some(
        u32::from_str_radix(&data.remove(0), 16).expect("Cannot convert index"),
    );
    if data.len()>0 {
        connected = Some(
            MatrixMixingStatus::from_str(&data.remove(0))
                .expect("Cannot convert value")
                .to_label(),
        )
    }
    (indx,ch,connected)
}