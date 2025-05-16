use std::str::FromStr;

use super::defs::errors::Error;
use super::defs::pan_tilt::{Direction, Velocity, DOWN, LEFT, RIGHT, UP,HOME};

pub fn move_camera(velocity: String, direction: String) -> Result<Vec<u8>, Error> {
    let Ok(velocity) = Velocity::from_str(&velocity) else {
        return Err(Error::InvalidVelocity);
    };
    let Ok(direction) = Direction::from_str(&direction) else {
        return Err(Error::InvalidDirection);
    };
    let coefficent:f32 = 33.0 / 100.0;
    let max_pan: f32 = 24.0;
    let max_tilt: f32 = 20.0;

    let (vv, ww) = match velocity {
        Velocity::SLOW => ((max_pan * coefficent).floor() as u8 * 1, (max_tilt * coefficent).floor() as u8 * 1),
        Velocity::MEDIUM => ((max_pan * coefficent).floor() as u8 * 2, (max_tilt * coefficent).floor() as u8 * 2),
        Velocity::FAST => ((max_pan * coefficent).floor() as u8 * 3, (max_tilt * coefficent).floor() as u8 * 3)
    };

    let mut direction = match direction {
        Direction::UP => UP,
        Direction::DOWN => DOWN,
        Direction::LEFT => LEFT,
        Direction::RIGHT => RIGHT,
        _ => return Err(Error::InvalidDirection)
    };
    let pan_condition = vv > 0 && vv <= max_pan as u8;
    let tilt_condition = ww > 0 && ww <= max_tilt as u8;
    if pan_condition && tilt_condition {
        direction[4] |= vv;
        direction[5] |= ww;
        Ok(direction.to_vec())
    } else {
        if !pan_condition {
            Err(Error::InvalidPan)
        } else {
            Err(Error::InvalidTilt)
        }
    }
}

pub fn return_home() -> Vec<u8>{
    HOME.to_vec()
}