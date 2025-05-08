use super::defs::camera_presets::RECALL;
use super::defs::errors::Error;


pub fn call_preset(pq:String)-> Result<Vec<u8>,Error>{
    let Ok(pq)= pq.parse::<u8>()else{
        return Err(Error::InvalidPreset);
    };
    let mut recall = RECALL;
    if pq <= 0x9 {recall[5] |= pq; Ok(recall.to_vec())} else { Err(Error::InvalidPreset)}
}