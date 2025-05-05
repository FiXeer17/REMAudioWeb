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

    pub const INVALID_STATUS_CODE_LABEL: &str = "Invalid status code";
    pub enum StatusCode{
        Accepted,
        Executed,
        SyntaxError,
        NotExecutable
    }
    impl TryFrom<&[u8]> for StatusCode{
        type Error = super::status_codes::Error;
        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let mut raw = value.to_vec();
            raw.remove(0);
            let status = &raw[..];
            match status {
                ACK_SUFFIX => Ok(Self::Accepted),
                COMPLETION_SUFFIX => Ok(Self::Executed),
                SYNTAX_ERROR_SUFFIX => Ok(Self::SyntaxError),
                COMMAND_NOT_EXECUTABLE_SUFFIX => Ok(Self::NotExecutable),
                _ => Err(Error::InvalidStatusCode)
            }
            
        }
    }

    #[derive(Debug,Clone,PartialEq)]
    pub enum Error{
        InvalidStatusCode
    }
    impl ToString for Error{
        fn to_string(&self) -> String {
            match self {
                Self::InvalidStatusCode => String::from(INVALID_STATUS_CODE_LABEL)
            }
        }
    }
}

pub mod camera_zoom{
    pub const STOP: [u8;6] = [0x08,0x01,0x04,0x07,0x00,0xff];
                                                            //
    pub const TELE_VARIABLE: [u8;6] = [0x08,0x01,0x04,0x07,0x20,0xff]; //the 4th byte has to be sum with the hex speed like 0x20 OR 0x04 -> 0x24
                                                            //
    pub const WIDE_VARIABLE: [u8;6] = [0x08,0x01,0x04,0x07,0x30,0xff]; //the 4th byte has to be sum with the hex speed like 0x30 OR 0x04 -> 0x34
    pub const INVALID_COEFFICIENT_LABEL :&str = "Invalid coefficient";

    
    #[derive(Debug,Clone,PartialEq)]
        pub enum Error{
            InvalidCoefficient
        }
        impl ToString for Error{
            fn to_string(&self) -> String {
                match self{
                    Self::InvalidCoefficient => String::from(INVALID_COEFFICIENT_LABEL)
                }
            }
        }
}

pub mod camera_presets{                                    //
    pub const RECALL : [u8;7] = [0x08,0x01,0x04,0x3F,0x02,0x00,0xff]; // the 5th byte has to be sum with the preset number like 0x00 OR 0x04 -> 0x04
    pub const INVALID_PRESET_LABEL: &str = "Invalid preset";

    #[derive(Debug,Clone,PartialEq)]
    pub enum Error{
        InvalidPreset
    }
    impl ToString for Error{
        fn to_string(&self) -> String {
            match self{
                Self::InvalidPreset => String::from(INVALID_PRESET_LABEL)
            }
        }
    }

}

pub mod pan_tilt{
                                                 //   //
    pub const UP: [u8;9] = [0x08,0x01,0x06,0x01,0x00,0x00,0x03,0x01,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04
                                                   //   //
    pub const DOWN: [u8;9] = [0x08,0x01,0x06,0x01,0x00,0x00,0x03,0x02,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04
                                                   //   //
    pub const LEFT: [u8;9] = [0x08,0x01,0x06,0x01,0x00,0x00,0x01,0x03,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04
                                                    //   //
    pub const RIGHT: [u8;9] = [0x08,0x01,0x06,0x01,0x00,0x00,0x02,0x03,0xff]; //the 4th and the 5th byte has to be sum with the pan and the tilt hex speed like 0x00 OR 0x04 -> 0x04

    pub const INVALID_PAN_LABEL :&str = "Invalid pan";
    pub const INVALID_TILT_LABEL :&str = "Invalid tilt";

    #[derive(Debug,Clone,PartialEq)]
    pub enum Direction{
        UP,
        DOWN,
        LEFT,
        RIGHT
    }

    #[derive(Debug,Clone,PartialEq)]
    pub enum Error{
        InvalidPan,
        InvalidTilt,
    }
    impl ToString for Error{
        fn to_string(&self) -> String {
            match self{
                Self::InvalidPan => String::from(INVALID_PAN_LABEL),
                Self::InvalidTilt=> String::from(INVALID_TILT_LABEL),
            }
        }
    }

}