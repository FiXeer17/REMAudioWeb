pub mod fncodes{
    use std::str::FromStr;

    pub const PRESETS_LABEL: &str = "camera_preset";
    pub const READ_PRESET_LABEL: &str = "read_preset";
    pub const ZOOM_TELE_LABEL: &str = "zoom_tele";
    pub const ZOOM_WIDE_LABEL: &str = "zoom_wide";
    pub const ZOOM_STOP_LABEL: &str = "zoom_stop";
    pub const MOVE_CAMERA_LABEL: &str = "move_camera";

    #[derive(Debug, Clone, PartialEq)]
    pub enum FNCODE {
        Preset,
        ZoomTele,
        ZoomWide,
        ZoomStop,
        MoveCamera,
        ReadPreset,
    }

  
    impl FNCODE {
        pub fn to_label(&self) -> String {
            match self {
                FNCODE::Preset => String::from(PRESETS_LABEL),
                FNCODE::ZoomTele => String::from(ZOOM_TELE_LABEL),
                FNCODE::ZoomWide => String::from(ZOOM_WIDE_LABEL),
                FNCODE::MoveCamera => String::from(MOVE_CAMERA_LABEL),
                FNCODE::ReadPreset => String::from(READ_PRESET_LABEL),
                FNCODE::ZoomStop => String::from(ZOOM_STOP_LABEL),
            }
        }
    }
    impl FromStr for FNCODE {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                PRESETS_LABEL => Ok(Self::Preset),
                ZOOM_TELE_LABEL => Ok(Self::ZoomTele),
                ZOOM_WIDE_LABEL => Ok(Self::ZoomWide),
                MOVE_CAMERA_LABEL => Ok(Self::MoveCamera),
                READ_PRESET_LABEL => Ok(Self::ReadPreset),
                ZOOM_STOP_LABEL => Ok(Self::ZoomStop),
                _ => Err(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CameraCommand{
    pub fncode : crate::engines::video_engine::defs::fncodes::FNCODE,
    pub cmd: Vec<u8>
}
pub mod errors{
    pub const INVALID_STATUS_CODE_LABEL: &str = "Invalid status code.";
    pub const TIMED_OUT_CODE_LABEL: &str = "Timed out.";
    pub const CLOSED_BY_REMOTE_CODE_LABEL: &str = "Closed by remote peer.";
    pub const INVALID_COEFFICIENT_LABEL :&str = "Invalid coefficient";
    pub const INVALID_PRESET_LABEL: &str = "Invalid preset";
    pub const INVALID_PAN_LABEL :&str = "Invalid pan";
    pub const INVALID_TILT_LABEL :&str = "Invalid tilt";
    pub const INVALID_DIRECTION_LABEL:&str = "Invalid direction";
    pub const INVALID_VELOCITY_LABEL:&str = "Invalid velocity";
    pub const INVALID_CMD_LABEL:&str = "Invalid command";

    #[derive(Debug,Clone,PartialEq)]
    pub enum Error{
        InvalidStatusCode,
        TimedOut,
        ClosedByRemotePeer,
        InvalidCoefficient,
        InvalidPreset,
        InvalidPan,
        InvalidTilt,
        InvalidDirection,
        InvalidVelocity,
        InvalidCmd,
    }
    impl ToString for Error{
        fn to_string(&self) -> String {
            match self {
                Self::InvalidStatusCode => String::from(INVALID_STATUS_CODE_LABEL),
                Self::ClosedByRemotePeer => String::from(CLOSED_BY_REMOTE_CODE_LABEL),
                Self::TimedOut => String::from(TIMED_OUT_CODE_LABEL),
                Self::InvalidCoefficient => String::from(INVALID_COEFFICIENT_LABEL),
                Self::InvalidPreset => String::from(INVALID_PRESET_LABEL),
                Self::InvalidPan => String::from(INVALID_PAN_LABEL),
                Self::InvalidTilt=> String::from(INVALID_TILT_LABEL),
                Self::InvalidDirection => String::from(INVALID_DIRECTION_LABEL),
                Self::InvalidVelocity => String::from(INVALID_VELOCITY_LABEL),
                Self::InvalidCmd => String::from(INVALID_CMD_LABEL),
            }
        }
    }
}

pub mod status_codes{
    pub const ACK_SUFFIX: &[u8]  = &[0x41,0xff]; // Returned when the command is accepted.
    pub const COMPLETION_SUFFIX: &[u8] = &[0x51, 0xff]; // Returned when the command has been executed.
    pub const SYNTAX_ERROR_SUFFIX: &[u8] = &[0x60,0x02,0xff]; 
    /*
        Returned when the command format is different
        or when a command with illegal command parameters is accepted
    */

    pub const COMMAND_NOT_EXECUTABLE_SUFFIX: &[u8] = &[0x61,0x41,0xff]; 
    /* 
        Returned when a command cannot be executed due to
        current conditions. For example, when commands
        controlling the focus manually are received during auto
        focus.
    */

   

    pub enum StatusCode{
        Accepted,
        Executed,
        SyntaxError,
        NotExecutable,
    }

    impl TryFrom<&[u8]> for StatusCode{
        type Error = super::errors::Error;
        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let mut raw = value.to_vec();
            raw.remove(0);
            let status = &raw[..];
            match status {
                ACK_SUFFIX => Ok(Self::Accepted),
                COMPLETION_SUFFIX => Ok(Self::Executed),
                SYNTAX_ERROR_SUFFIX => Ok(Self::SyntaxError),
                COMMAND_NOT_EXECUTABLE_SUFFIX => Ok(Self::NotExecutable),
                _ => Err(super::errors::Error::InvalidStatusCode)
            }
            
        }
    }


}

pub mod camera_zoom{
    pub const ZOOM_STOP: [u8;6] = [0x81,0x01,0x04,0x07,0x00,0xff];
                                                            //
    pub const ZOOM_TELE: [u8;6] = [0x81,0x01,0x04,0x07,0x02,0xff]; //the 4th byte has to be sum with the hex speed like 0x20 OR 0x04 -> 0x24
                                                            //
    pub const ZOOM_WIDE: [u8;6] = [0x81,0x01,0x04,0x07,0x03,0xff]; //the 4th byte has to be sum with the hex speed like 0x30 OR 0x04 -> 0x34

    

}

pub mod camera_presets{                                    //
    pub const RECALL : [u8;7] = [0x81,0x01,0x04,0x3F,0x02,0x00,0xff]; // the 5th byte has to be sum with the preset number like 0x00 OR 0x04 -> 0x04
    pub const READ_PRESET : [u8;5] = [0x81,0x09,0x04,0x3f,0xff];

}   

pub mod pan_tilt{
    use std::str::FromStr;

                                                 //   //
    pub const UP: [u8;9] = [0x81,0x01,0x06,0x01,0x00,0x00,0x03,0x01,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04
                                                   //   //
    pub const DOWN: [u8;9] = [0x81,0x01,0x06,0x01,0x00,0x00,0x03,0x02,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04
                                                   //   //
    pub const LEFT: [u8;9] = [0x81,0x01,0x06,0x01,0x00,0x00,0x01,0x03,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04
                                                    //   //
    pub const RIGHT: [u8;9] = [0x81,0x01,0x06,0x01,0x00,0x00,0x02,0x03,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04

    pub const HOME: [u8;5]= [0x81, 0x01, 0x06, 0x04, 0xff];

    pub const UP_LABEL: &str = "up";
    pub const DOWN_LABEL: &str = "down";
    pub const LEFT_LABEL: &str = "left";
    pub const RIGHT_LABEL:&str = "right";
    pub const HOME_LABEL: &str = "home";
    pub const SLOW_LABEL: &str = "slow";
    pub const MEDIUM_LABEL: &str = "medium";
    pub const FAST_LABEL: &str = "fast";

    #[derive(Debug,Clone,PartialEq)]
    pub enum Direction{
        UP,
        DOWN,
        LEFT,
        RIGHT,
        HOME
    }

    #[derive(Debug,Clone,PartialEq)]
    pub enum Velocity{
        SLOW,
        MEDIUM,
        FAST
    }

    impl ToString for Velocity{
        fn to_string(&self) -> String {
            match self {
                Self::SLOW => String::from(SLOW_LABEL),
                Self::MEDIUM => String::from(MEDIUM_LABEL),
                Self::FAST => String::from(FAST_LABEL)
            }
        }
    }

    impl FromStr for Velocity{
        type Err = super::errors::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                SLOW_LABEL => Ok(Self::SLOW),
                MEDIUM_LABEL => Ok(Self::MEDIUM),
                FAST_LABEL => Ok(Self::FAST),
                _ => Err(super::errors::Error::InvalidVelocity)
            }
        }
    }
    impl ToString for Direction{
        fn to_string(&self) -> String {
            match self {
                Self::UP => String::from(UP_LABEL),
                Self::DOWN => String::from(DOWN_LABEL),
                Self::LEFT => String::from(LEFT_LABEL),
                Self::RIGHT => String::from(RIGHT_LABEL),
                Self::HOME => String::from(HOME_LABEL)
            }
        }
    }
    impl FromStr for Direction{
        type Err = super::errors::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                UP_LABEL => Ok(Self::UP),
                DOWN_LABEL => Ok(Self::DOWN),
                LEFT_LABEL => Ok(Self::LEFT),
                RIGHT_LABEL => Ok(Self::RIGHT),
                _ => Err(super::errors::Error::InvalidDirection)

            }
        }
    }

}


