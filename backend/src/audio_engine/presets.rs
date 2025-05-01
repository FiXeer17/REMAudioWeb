use crate::audio_engine::defs;
use crate::audio_engine::defs::fncodes;
use crate::audio_engine::lib::MatrixCommand;
use crate::services::private::app::schemas::SetState;

use super::defs::errors::Error;

pub fn read_current_preset() -> Result<MatrixCommand, Error> {
    let rw = defs::datas::rw::READ.to_string();
    let fcode = fncodes::SCENE.to_string();

    MatrixCommand::new(rw, fcode, None)
}


pub fn into_data(data: SetState) ->Result<Vec<String>,Error>{
    let value = data.value.unwrap();
                match value.parse::<u16>() {
                    Ok(v) => {
                        if v > 16 || v == 0 {
                            return Err(Error::InvalidPreset);
                        }
                        Ok(vec![format!("{:02}", v)])
                    }
                    Err(e) => return Err(Error::ConversionError(e.to_string())),
                }
}