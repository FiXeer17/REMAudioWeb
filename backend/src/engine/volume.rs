use crate::engine::defs;
use crate::engine::defs::{datas::io, fncodes};
use crate::engine::lib::MatrixCommand;

pub fn read_volume_ch(src: io::SRC, ch: u32) -> Result<MatrixCommand, String> {
    let fcode = fncodes::VOLUME.to_string();
    let rw = defs::datas::rw::READ.to_string();
    let io = src.to_string();
    if ch < 1 || ch > 16 {
        return Err("Invalid channel".to_string());
    }
    let ch = format!("{:02}", ch);

    let data = Some(vec![io, ch]);

    MatrixCommand::new(rw, fcode, data)
}

pub fn read_volume_all(src: io::SRC) -> Result<Vec<MatrixCommand>, String> {
    let fcode = fncodes::VOLUME.to_string();
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
