use std::str::FromStr;

use crate::audio_engine::defs;
use crate::audio_engine::defs::{datas::io, fncodes};
use crate::audio_engine::lib::MatrixCommand;
use crate::services::private::app::schemas::SetState;
use super::defs::datas::io::SRC;
use super::defs::datas::mute_status::MuteStatus;
use super::defs::errors::Error;

pub fn read_mute_ch(src: io::SRC, ch: u32) -> Result<MatrixCommand, Error> {
    let fcode = fncodes::MUTE.to_string();
    let rw = defs::datas::rw::READ.to_string();
    let io = src.to_string();
    if ch < 1 || ch > 16 {
        return Err(Error::InvalidChannel);
    }
    let ch = format!("{:02}", ch);

    let data = Some(vec![io, ch]);

    MatrixCommand::new(rw, fcode, data)
}
pub fn read_mute_all(src: io::SRC) -> Result<Vec<MatrixCommand>, Error> {
    let fcode = fncodes::MUTE.to_string();
    let rw = defs::datas::rw::READ.to_string();
    let io = src.to_string();
    let mut commands: Vec<MatrixCommand> = Vec::new();
    for ch in 1..=16 {
        let ch = format!("{:02}", ch);
        let data = Some(vec![io.clone(), ch]);
        commands.push(MatrixCommand::new(rw.clone(), fcode.clone(), data).unwrap());
    }
    Ok(commands)
}


pub fn into_data(data: SetState) ->Result<Vec<String>,Error> {
    let io = SRC::from_str(data.io.unwrap().as_str())?;
    let channel = format!("{:02}", MatrixCommand::check_channel(data.channel.unwrap())?);
    let value = MuteStatus::from_str(data.value.unwrap().as_str())?;
    Ok(vec![io.to_string(), channel, value.to_string()])
}