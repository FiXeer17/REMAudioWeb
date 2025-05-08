use super::defs::camera_presets::{Error,RECALL};


pub fn call_preset(pq:u8)-> Result<[u8;7],Error>{
    let mut recall = RECALL;
    if pq <= 0x9 {recall[5] |= pq; Ok(recall)} else { Err(Error::InvalidPreset)}
}