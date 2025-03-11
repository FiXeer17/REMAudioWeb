use crate::engine::{
    defs::{datas::io::SRC, fncodes::FNCODE, *},
    mute::read_mute_all,
    presets::read_current_preset,
    volume::read_volume_all,
};

use core::fmt;
use std::str::FromStr;

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

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixCommand {
    pub start: String,
    pub id: String,
    pub rw: String,
    pub fcode: String,
    pub data_length: Option<String>,
    pub data: Option<Vec<String>>,
    pub end: String,
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

impl fmt::Display for MatrixCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            Some(value) => {
                let d = value.join(" ");
                write!(
                    f,
                    "{} {} {} {} {} {} {}",
                    self.start,
                    self.id,
                    self.rw,
                    self.fcode,
                    self.data_length.as_deref().unwrap(),
                    d,
                    self.end
                )
            }
            None => {
                write!(
                    f,
                    "{} {} {} {} {} {}",
                    self.start,
                    self.id,
                    self.rw,
                    self.fcode,
                    self.data_length.as_deref().unwrap_or("00"),
                    self.end
                )
            }
        }
    }
}

impl FromStr for MatrixCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix(format!("{} {}", START_CODE, "FF").as_str()) {
            Some(raw_without_prefix) => match raw_without_prefix.strip_suffix(END_CODE) {
                Some(raw) => {
                    let mut raw_vec: Vec<&str> = raw.split_whitespace().collect();
                    raw_vec.retain(|&value| !value.is_empty());

                    let rw = raw_vec.get(0).ok_or_else(|| String::from("Missing rw"))?;
                    let fcode = raw_vec
                        .get(1)
                        .ok_or_else(|| String::from("Missing fcode"))?;
                    let data_length = raw_vec
                        .get(2)
                        .ok_or_else(|| String::from("Missing data length"))?;

                    if let Ok(usable_data_length) = u8::from_str_radix(data_length, 10) {
                        let data: Option<Vec<String>>;
                        let mut data_length = Some(data_length.to_string());

                        if usable_data_length > 0 {
                            let data_slice = &raw_vec[3..];
                            data = Some(data_slice.iter().map(|&s| s.to_string()).collect());
                        } else {
                            data = None;
                            data_length = None;
                        }

                        return Ok(MatrixCommand {
                            start: START_CODE.to_string(),
                            id: "FF".to_string(),
                            rw: rw.to_string(),
                            fcode: fcode.to_string(),
                            data_length,
                            data,
                            end: END_CODE.to_string(),
                        });
                    } else {
                        return Err(String::from("Invalid format, data length isn't a number"));
                    }
                }
                None => Err(String::from("Invalid format, no end code found")),
            },
            None => Err(String::from("Invalid format, no start code found")),
        }
    }
}

pub fn read_all_states() -> Result<Vec<MatrixCommand>, String> {
    let (in_volume_states, out_volume_sates) =
        (read_volume_all(SRC::INPUT)?, read_volume_all(SRC::OUTPUT)?);
    let (in_mute_states, out_mute_states) =
        (read_mute_all(SRC::INPUT)?, read_mute_all(SRC::OUTPUT)?);
    let current_preset = read_current_preset()?;
    let mut commands: Vec<MatrixCommand> = Vec::with_capacity(
        in_volume_states.len()
            + out_volume_sates.len()
            + in_mute_states.len()
            + out_mute_states.len()
            + 1,
    );
    commands.extend_from_slice(&in_mute_states[..]);
    commands.extend_from_slice(&out_mute_states[..]);
    commands.extend_from_slice(&in_volume_states[..]);
    commands.extend_from_slice(&out_volume_sates[..]);
    commands.push(current_preset);

    Ok(commands)
}
