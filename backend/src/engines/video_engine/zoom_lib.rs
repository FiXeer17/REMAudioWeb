use super::defs::camera_zoom::{TELE_VARIABLE,WIDE_VARIABLE};
use super::defs::errors::Error;



pub fn zoom_tele(p:String)-> Result<Vec<u8>,Error>{
    let Ok(p) = p.parse::<u8>() else{
        return Err(Error::InvalidCoefficient)
    };
    let mut tele = TELE_VARIABLE;
    if p <= 0xF {tele[4] |= p; Ok(tele.to_vec()) } else {Err(Error::InvalidCoefficient)}
}

pub fn zoom_wide(p:String) -> Result<Vec<u8>,Error>{
    let Ok(p) = p.parse::<u8>() else{
        return Err(Error::InvalidCoefficient)
    };
    let mut wide = WIDE_VARIABLE;
    if p <= 0xF {wide[4] |= p; Ok(wide.to_vec())} else {Err(Error::InvalidCoefficient)}
}