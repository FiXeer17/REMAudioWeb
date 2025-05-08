use std::str::FromStr;

use crate::engines::audio_engine::defs;
use crate::engines::audio_engine::defs::{datas::io, fncodes};
use crate::engines::audio_engine::lib::MatrixCommand;
use crate::configs::channels_settings;
use crate::services::private::app::schemas::SetState;
use super::defs::datas::io::SRC;
use super::defs::datas::mute_status::MuteStatus;
use super::defs::errors::Error;

pub fn read_mute_ch(src: io::SRC, ch: u32) -> Result<MatrixCommand, Error> {
    let fcode = fncodes::MUTE.to_string();
    let rw = defs::datas::rw::READ.to_string();
    let io = src.to_string();
    let ch = format!("{:02X}", ch);
    MatrixCommand::check_channel(&ch)?;

    let data = Some(vec![io, ch]);

    MatrixCommand::new(rw, fcode, data)
}
pub fn read_mute_all(src: io::SRC) -> Result<Vec<MatrixCommand>, Error> {
    let fcode = fncodes::MUTE.to_string();
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


pub fn into_data(data: SetState) ->Result<Vec<String>,Error> {
    let io = SRC::from_str(data.io.unwrap().as_str())?;
    let channel = format!("{:02X}", data.channel.unwrap().trim().parse::<u8>().unwrap());
    MatrixCommand::check_channel(&channel)?;
    let value = MuteStatus::from_str(data.value.unwrap().as_str())?;
    Ok(vec![io.to_string(), channel, value.to_string()])
}

pub fn into_deserialized(mut data:Vec<String>) -> (Option<String>,Option<u32>,Option<bool>){
    let io = Some(
        SRC::from_str(&data.remove(0))
            .expect("Cannot retrieve io code")
            .to_label(),
    );
    let ch = Some(
        u32::from_str_radix(&data.remove(0), 16).expect("Cannot find channel code"),
    );
    let value = data.remove(0);
    let status = MuteStatus::from_str(&value).expect("Cannot convert mute code");
    let muted = Some(status.to_label());
    
    (io,ch,muted)
}