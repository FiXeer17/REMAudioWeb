#[allow(dead_code)]

// here are defined all constants used inside the engine:

pub const START_CODE: &str = "A5 C3 3C 5A";
pub const END_CODE: &str = "EE";
pub const STEP_UNIT: f32 = 0.1;

// FUNCTION CODES
pub mod fncodes {
    use std::str::FromStr;

    pub const SCENE: &str = "02";
    pub const MUTE: &str = "03";
    pub const VOLUME: &str = "04";
    pub const GAIN_IN_STEP: &str = "05";
    pub const MIC_SENSITIVITY: &str = "06";
    pub const MATRIX_MIXING:&str = "09";

    pub const SCENE_LABEL: &str = "preset";
    pub const MUTE_LABEL: &str = "mute";
    pub const VOLUME_LABEL: &str = "volume";
    pub const GAIN_IN_STEP_LABEL: &str = "gain_in_step";
    pub const MIC_SENSITIVITY_LABEL: &str = "mic_sensitivity";
    pub const MATRIX_MIXING_LABEL: &str = "matrix_mixing";
    pub enum FNCODE {
        SCENE,
        MUTE,
        VOLUME,
        GAINSTEP,
        MICSENSITIVITY,
        MATRIXMIXING
    }

  
    impl ToString for FNCODE{
        fn to_string(&self) -> String {
            match self{
                FNCODE::SCENE => String::from(SCENE),
                FNCODE::MUTE => String::from(MUTE),
                FNCODE::VOLUME => String::from(VOLUME),
                FNCODE::GAINSTEP => String::from(GAIN_IN_STEP),
                FNCODE::MICSENSITIVITY => String::from(MIC_SENSITIVITY),
                FNCODE::MATRIXMIXING => String::from(MATRIX_MIXING)
            }
        }
    }
    impl FNCODE {
        pub fn to_label(&self) -> String {
            match self {
                FNCODE::SCENE => String::from(SCENE_LABEL),
                FNCODE::MUTE => String::from(MUTE_LABEL),
                FNCODE::VOLUME => String::from(VOLUME_LABEL),
                FNCODE::GAINSTEP => String::from(GAIN_IN_STEP_LABEL),
                FNCODE::MICSENSITIVITY => String::from(MIC_SENSITIVITY_LABEL),
                FNCODE::MATRIXMIXING => String::from(MATRIX_MIXING_LABEL)
            }
        }
    }
    impl FromStr for FNCODE {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                SCENE => Ok(FNCODE::SCENE),
                MUTE => Ok(FNCODE::MUTE),
                VOLUME => Ok(FNCODE::VOLUME),
                GAIN_IN_STEP => Ok(FNCODE::GAINSTEP),
                MIC_SENSITIVITY => Ok(FNCODE::MICSENSITIVITY),
                MATRIX_MIXING => Ok(FNCODE::MATRIXMIXING),
                SCENE_LABEL => Ok(FNCODE::SCENE),
                MUTE_LABEL => Ok(FNCODE::MUTE),
                VOLUME_LABEL => Ok(FNCODE::VOLUME),
                GAIN_IN_STEP_LABEL => Ok(FNCODE::GAINSTEP),
                MIC_SENSITIVITY_LABEL => Ok(FNCODE::MICSENSITIVITY),
                MATRIX_MIXING_LABEL => Ok(FNCODE::MATRIXMIXING),
                _ => Err(())
            }
        }
    }
}

pub mod sections{
    use std::str::FromStr;

    use super::{errors, fncodes::*};

    pub const VISIBILITY_LABEL :&str = "visibility";
    pub const CHANNEL_LABELS_LABEL : &str = "channel_labels";
    pub const PRESET_LABELS_LABEL : &str = "preset_labels";

    pub enum Sections{
        Visibility,
        ChannelLabels,
        PresetLabels,
        MatrixCommand(FNCODE),
    }

    impl ToString for Sections{
        fn to_string(&self) -> String {
            match self{
                Sections::ChannelLabels => String::from(CHANNEL_LABELS_LABEL),
                Sections::PresetLabels => String::from(PRESET_LABELS_LABEL),
                Sections::Visibility => String::from(VISIBILITY_LABEL),
                Sections::MatrixCommand(cmd) => cmd.to_label(),
            }
        }
    }
    impl FromStr for Sections {
        type Err = errors::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                VISIBILITY_LABEL => Ok(Sections::Visibility),
                CHANNEL_LABELS_LABEL => Ok(Sections::ChannelLabels),
                PRESET_LABELS_LABEL => Ok(Sections::PresetLabels),
                VOLUME_LABEL => Ok(Sections::MatrixCommand(FNCODE::VOLUME)),
                MUTE_LABEL => Ok(Sections::MatrixCommand(FNCODE::MUTE)),
                SCENE_LABEL => Ok(Sections::MatrixCommand(FNCODE::SCENE)),
                _ => Err(errors::Error::InvalidSection)
            }
        }
    }
}

// DATA FOUNDAMENTALS:
pub mod datas {
    // INPUT/OUTPUT IDs
    pub mod io {
        use core::fmt;
        use std::str::FromStr;

        use crate::audio_engine::defs::errors::Error;

        pub const GENERAL: &str = "00";
        pub const INPUT: &str = "01";
        pub const OUTPUT: &str = "02";
        pub const GENERAL_LABEL: &str = "both";
        pub const INPUT_LABEL: &str = "input";
        pub const OUTPUT_LABEL: &str = "output";

        pub enum SRC {
            GENERAL,
            INPUT,
            OUTPUT,
        }

        impl SRC {
            pub fn to_label(&self) -> String {
                match self {
                    SRC::GENERAL => String::from(GENERAL_LABEL),
                    SRC::INPUT => String::from(INPUT_LABEL),
                    SRC::OUTPUT => String::from(OUTPUT_LABEL),
                }
            }
        }

        impl From<SRC> for &'static str {
            fn from(value: SRC) -> Self {
                match value {
                    SRC::GENERAL => GENERAL,
                    SRC::INPUT => INPUT,
                    SRC::OUTPUT => OUTPUT,
                }
            }
        }

        impl FromStr for SRC {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    INPUT => Ok(SRC::INPUT),
                    OUTPUT => Ok(SRC::OUTPUT),
                    GENERAL => Ok(SRC::GENERAL),
                    INPUT_LABEL => Ok(SRC::INPUT),
                    OUTPUT_LABEL => Ok(SRC::OUTPUT),
                    GENERAL_LABEL => Ok(SRC::GENERAL),
                    _ => Err(Error::ConversionError("cannot convert source".to_string())),
                }
            }
        }

        impl fmt::Display for SRC {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    &SRC::GENERAL => write!(f, "{}", GENERAL),
                    &SRC::INPUT => write!(f, "{}", INPUT),
                    &SRC::OUTPUT => write!(f, "{}", OUTPUT),
                }
            }
        }
    }
    // READ/WRITE IDs
    pub mod rw {
        pub const READ: &str = "63";
        pub const WRITE: &str = "36";
    }
    pub mod mute_status{
        use std::str::FromStr;

        use crate::audio_engine::defs::errors::Error;

        pub const NOTMUTED: &str = "00";
        pub const MUTED: &str = "01";
        pub const NOTMUTED_LABEL: &str = "false";
        pub const MUTED_LABEL: &str = "true";

        #[derive(Debug,Clone)]
        pub enum MuteStatus{
            MUTED,
            NOTMUTED,
        }
        impl MuteStatus{
            pub fn to_label(&self) -> bool {
                match self {
                     MuteStatus::MUTED => bool::from_str(MUTED_LABEL).unwrap(),
                     MuteStatus::NOTMUTED => bool::from_str(NOTMUTED_LABEL).unwrap(),
                }
            }
        }

        impl FromStr for MuteStatus{
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    MUTED => Ok(MuteStatus::MUTED),
                    NOTMUTED => Ok(MuteStatus::NOTMUTED),
                    MUTED_LABEL => Ok(MuteStatus::MUTED),
                    NOTMUTED_LABEL=> Ok(MuteStatus::NOTMUTED),
                    _ => Err(Error::ConversionError("cannot convert mute status.".to_string()))
                }
            }
        }

        impl ToString for MuteStatus{
            fn to_string(&self) -> String {
                match self{
                    MuteStatus::MUTED => MUTED.to_string(),
                    MuteStatus::NOTMUTED => NOTMUTED.to_string(), 
                }
            }
        }
    }
    pub mod matrix_mixing_status{
        use std::str::FromStr;

        use crate::audio_engine::defs::errors::Error;

        pub const CONNECTED:&str = "01";
        pub const DISCONNECTED: &str = "00";
        pub const CONNECTED_LABEL:&str = "connected";
        pub const DISCONNECTED_LABEL:&str = "disconnected";

        #[derive(Debug,Clone)]
        pub enum MatrixMixingStatus{
            CONNECTED,
            DISCONNECTED,
        }

        impl MatrixMixingStatus{
            pub fn to_label(&self) -> bool {
                match self {
                     MatrixMixingStatus::CONNECTED => true,
                     MatrixMixingStatus::DISCONNECTED => false,
                }
            }
        }

        impl FromStr for MatrixMixingStatus{
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    CONNECTED => Ok(MatrixMixingStatus::CONNECTED),
                    DISCONNECTED => Ok(MatrixMixingStatus::DISCONNECTED),
                    CONNECTED_LABEL => Ok(MatrixMixingStatus::CONNECTED),
                    DISCONNECTED_LABEL=> Ok(MatrixMixingStatus::DISCONNECTED),
                    _ => Err(Error::ConversionError("cannot convert matrix mixing status.".to_string()))
                }
            }
        }
        impl ToString for MatrixMixingStatus{
            fn to_string(&self) -> String {
                match self{
                    MatrixMixingStatus::CONNECTED => CONNECTED.to_string(),
                    MatrixMixingStatus::DISCONNECTED => DISCONNECTED.to_string(), 
                }
            }
        }

    }
}
// STATUS CODES RETURNING FROM MATRIX
pub mod status_codes {
    use super::errors::Error;


    pub const SUCCESS: &str = "00";
    pub const ERR: &str = "01";

    pub enum StatusCodes{
        Success,
        Error
    }
    
    impl ToString for StatusCodes{
        fn to_string(&self) -> String {
            match self {
                StatusCodes::Success => SUCCESS.to_string(),
                StatusCodes::Error => ERR.to_string()
            }
        }
    }
    impl TryFrom<&[u8]> for StatusCodes{
        type Error = Error;
        fn try_from(value: &[u8]) -> Result<Self, Error> {
            if value.len() > 2{
                
                return Err(Error::InvalidCode)
            } 
            let value = String::from_utf8(value.to_vec()).unwrap();
            
            match value.as_str() {
                SUCCESS => Ok(StatusCodes::Success),
                ERR => Ok(StatusCodes::Error),
                _ => return Err(Error::InvalidCode)

            }
        }
    }
}

pub mod errors{
    #[derive(Debug,Clone)]
    pub enum Error{
        ConversionError(String),
        InvalidCode,
        InvalidFormat(String),
        InvalidChannel,
        InvalidData(String),
        InvalidPreset,
        InvalidFunctionCode,
        InvalidSection,
    }

    impl ToString for Error{
        fn to_string(&self) -> String {
            match self {
                Error::ConversionError(value) => String::from(value),
                Error::InvalidCode => String::from("invalid code"),
                Error::InvalidFormat(value) => String::from(value),
                Error::InvalidChannel => String::from("invalid channel"),
                Error::InvalidData(value) => String::from(value),
                Error::InvalidPreset => String::from("invalid preset"),
                Error::InvalidFunctionCode => String::from("invalid function code"),
                Error::InvalidSection => String::from("invalid section")

            }
        }
    }
}