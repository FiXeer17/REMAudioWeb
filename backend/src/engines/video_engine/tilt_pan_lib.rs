use std::str::FromStr;

use super::defs::errors::Error;
use super::defs::pan_tilt::{Direction, Velocity, DOWN, LEFT, RIGHT, UP};

pub fn move_camera(velocity: String, direction: String) -> Result<Vec<u8>, Error> {
    let Ok(velocity) = Velocity::from_str(&velocity) else {
        return Err(Error::InvalidVelocity);
    };
    let Ok(direction) = Direction::from_str(&direction) else {
        return Err(Error::InvalidDirection);
    };
    let coefficent = 33 / 100;
    let max_pan = 0x18;
    let max_tilt = 0x14;

    let (vv, ww) = match velocity {
        Velocity::SLOW => (max_pan * coefficent * 1, max_tilt * coefficent * 1),
        Velocity::MEDIUM => (max_pan * coefficent * 2, max_tilt * coefficent * 2),
        Velocity::FAST => (max_pan * coefficent * 3, max_tilt * coefficent * 3)
    };

    let mut direction = match direction {
        Direction::UP => UP,
        Direction::DOWN => DOWN,
        Direction::LEFT => LEFT,
        Direction::RIGHT => RIGHT,
    };

    let pan_condition = vv > 0 && vv <= max_pan;
    let tilt_condition = ww > 0 && ww <= max_tilt;
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
