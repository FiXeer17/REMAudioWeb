use std::str::FromStr;

use crate::engines::audio_engine::defs;
use crate::engines::audio_engine::defs::{datas::io, fncodes};
use crate::engines::audio_engine::lib::MatrixCommand;
use crate::configs::channels_settings;
use crate::services::private::app::schemas::SetState;

use super::defs::datas::io::SRC;
use super::defs::errors::Error;
use super::defs::STEP_UNIT;

pub fn read_volume_ch(src: io::SRC, ch: u32) -> Result<MatrixCommand, Error> {
    let fcode = fncodes::VOLUME.to_string();
    let rw = defs::datas::rw::READ.to_string();
    let io = src.to_string();
    if ch > channels_settings::get_channels_number() as u32 {
        return Err(Error::InvalidChannel);
    }
    let ch = format!("{:02X}", ch);

    let data = Some(vec![io, ch]);

    MatrixCommand::new(rw, fcode, data)
}

pub fn read_volume_all(src: io::SRC) -> Result<Vec<MatrixCommand>, Error> {
    let fcode = fncodes::VOLUME.to_string();
    let rw = defs::datas::rw::READ.to_string();
    let io = src.to_string();
    let mut commands: Vec<MatrixCommand> = Vec::new();
    for ch in 1..=channels_settings::get_channels_number() {
        let ch = format!("{:02X}", ch);
        let data = Some(vec![io.clone(), ch]);
        commands.push(MatrixCommand::new(rw.clone(), fcode.clone(), data).unwrap());
    }
    Ok(commands)
}

pub fn into_data(data: SetState) -> Result<Vec<String>, Error> {
    let io = SRC::from_str(data.io.unwrap().as_str())?;
    let channel = format!("{:02X}",data.channel.unwrap().trim().parse::<u8>().unwrap());
    MatrixCommand::check_channel(&channel)?;
    let value = data.value.unwrap();
    match value.parse::<f32>() {
        Ok(v) => {
            let volume = v / STEP_UNIT;
            let volume = format!("{:04X}", volume as i16 as u16);
            let hexvol = volume.split_at(2);
            Ok(vec![
                io.to_string(),
                channel,
                hexvol.1.to_string(),
                hexvol.0.to_string(),
            ])
        }
        Err(e) => return Err(Error::ConversionError(e.to_string())),
    }
}


pub fn into_deserialized(mut data:Vec<String>) -> (Option<String>,Option<u32>,Option<f32>){
    let mut v = None; 
    let io = Some(
        SRC::from_str(&data.remove(0))
            .expect("Cannot retrieve io code")
            .to_label(),
    );
    let ch = Some(
        u32::from_str_radix(&data.remove(0), 16).expect("Cannot find channel code"),
    );
    if data.len()>0{
        data.reverse();
        let decimal = u16::from_str_radix(&data.concat(), 16)
            .expect("Cannot convert data code")
            as i16;
        v = Some(decimal as f32 * STEP_UNIT)
    }
    
    (io,ch,v)
}