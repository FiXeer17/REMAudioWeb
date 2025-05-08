
use std::str::FromStr;

use super::{
    defs::{datas::matrix_mixing_status::MatrixMixingStatus, errors::Error},
    lib::MatrixCommand,
};
use crate::services::private::app::schemas::SetState;

pub fn into_data(data: SetState) -> Result<Vec<String>, Error> {
    let index = format!(
        "{:02X}",
        data.index.unwrap().trim().parse::<u8>().unwrap()
    );
    let channel = format!(
        "{:02X}",
        data.channel.unwrap().trim().parse::<u8>().unwrap()
    );
    MatrixCommand::check_channel(channel.clone())?;
    MatrixCommand::check_channel(index.clone())?;
    
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