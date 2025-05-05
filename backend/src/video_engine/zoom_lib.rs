use super::defs::camera_zoom::{Error,TELE_VARIABLE,WIDE_VARIABLE};


pub fn zoom_tele(p:u8)-> Result<[u8;6],Error>{
    let mut tele = TELE_VARIABLE;
    if p <= 0xF {tele[4] |= p; Ok(tele) } else {Err(Error::InvalidCoefficient)}
}

pub fn zoom_wide(p:u8) -> Result<[u8;6],Error>{
    let mut wide = WIDE_VARIABLE;
    if p <= 0xF {wide[4] |= p; Ok(wide)} else {Err(Error::InvalidCoefficient)}
}