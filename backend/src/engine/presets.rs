use crate::engine::defs;
use crate::engine::defs::fncodes;
use crate::engine::lib::MatrixCommand;

pub fn read_current_preset() -> Result<MatrixCommand, String> {
    let rw = defs::datas::rw::READ.to_string();
    let fcode = fncodes::SCENE.to_string();

    MatrixCommand::new(rw, fcode, None)
}
