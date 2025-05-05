use super::defs::pan_tilt::{Direction, Error,UP, DOWN, LEFT, RIGHT};


pub fn move_camera(vv:u8,ww:u8,direction:Direction) -> Result<[u8;9],Error>{
    let mut up = match direction{
        Direction::UP => UP,
        Direction::DOWN => DOWN,
        Direction::LEFT => LEFT,
        Direction::RIGHT => RIGHT
    };

    let pan_condition = vv>0 && vv<=0x18;
    let tilt_condition = ww>0 && ww<=0x14;
    if pan_condition && tilt_condition {up[4]|=vv; up[5]|=ww; Ok(up)} else {if !pan_condition{Err(Error::InvalidPan)} else {Err(Error::InvalidTilt)}}
}

