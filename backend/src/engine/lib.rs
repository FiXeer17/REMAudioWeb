use core::fmt;
use std::str::FromStr;

use crate::engine::defs::{fncodes::FNCODE, *};
pub trait Command {
    fn is_valid_format(&self) -> bool;
}

impl Command for String {
    fn is_valid_format(&self) -> bool {
        if self.len() == 2 && self.chars().all(|c| c.is_digit(16)) {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct MatrixCommand {
    pub start: String,
    pub id: String,
    pub rw: String,
    pub fcode: String,
    pub data_length: Option<String>,
    pub data: Option<Vec<String>>,
    pub end: String,
}

impl fmt::Display for MatrixCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match &self.data {
            Some(value) => value.join(" "),
            None => "".to_string(),
        };

        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.start,
            self.id,
            self.rw,
            self.fcode,
            self.data_length.as_deref().unwrap_or(""),
            data,
            self.end
        )
    }
}

impl MatrixCommand {
    pub fn new(rw: String, fcode: String, data: Option<Vec<String>>) -> Result<Self, String> {
        if !rw.is_valid_format() || !fcode.is_valid_format() || FNCODE::from_str(&fcode).is_err() {
            return Err("Format not valid".to_string());
        }

        let mut data_length: Option<String> = None;

        if let Some(ref vec) = data {
            if !vec.iter().all(|val| val.is_valid_format()) {
                return Err("Invalid data format".to_string());
            }
            data_length = Some(format!("{:02}", vec.len()));
        }

        Ok(MatrixCommand {
            start: START_CODE.to_string(),
            id: "FF".to_string(),
            rw,
            fcode,
            data_length,
            data,
            end: END_CODE.to_string(),
        })
    }
}

pub mod mute {
    use crate::engine::defs;
    use crate::engine::defs::{datas::io, fncodes};
    use crate::engine::lib::MatrixCommand;

    pub fn read_mute_ch(src: io::SRC, ch: u32) -> Result<MatrixCommand, String> {
        let fcode = fncodes::MUTE.to_string();
        let rw = defs::datas::rw::READ.to_string();
        let io = src.to_string();
        if ch < 1 || ch > 16 {
            return Err("Invalid channel".to_string());
        }
        let ch = format!("{:02}", ch);

        let data = Some(vec![io, ch]);

        MatrixCommand::new(rw, fcode, data)
    }
    pub fn read_mute_all(src: io::SRC) -> Result<Vec<MatrixCommand>, String> {
        let fcode = fncodes::MUTE.to_string();
        let rw = defs::datas::rw::READ.to_string();
        let io = src.to_string();
        let mut commands: Vec<MatrixCommand> = Vec::new();
        for ch in 1..=16 {
            let ch = format!("{:02}",ch);
            let data = Some(vec![io.clone(), ch]);
            commands.push(MatrixCommand::new(rw.clone(), fcode.clone(), data).unwrap());
        }
        Ok(commands)
    }
}
