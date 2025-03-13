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

    pub enum FNCODE {
        SCENE,
        MUTE,
        VOLUME,
        GAINSTEP,
        MICSENSITIVITY,
    }

    impl From<FNCODE> for &'static str {
        fn from(code: FNCODE) -> Self {
            match code {
                FNCODE::SCENE => SCENE,
                FNCODE::MUTE => MUTE,
                FNCODE::VOLUME => VOLUME,
                FNCODE::GAINSTEP => GAIN_IN_STEP,
                FNCODE::MICSENSITIVITY => MIC_SENSITIVITY,
            }
        }
    }
    impl FNCODE {
        pub fn to_label(&self) -> String {
            match self {
                FNCODE::SCENE => String::from("preset"),
                FNCODE::MUTE => String::from("mute"),
                FNCODE::VOLUME => String::from("volume"),
                FNCODE::GAINSTEP => String::from("gain_in_step"),
                FNCODE::MICSENSITIVITY => String::from("mic_sensitivity"),
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
                _ => Err(()),
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

        pub const GENERAL: &str = "00";
        pub const INPUT: &str = "01";
        pub const OUTPUT: &str = "02";

        pub enum SRC {
            GENERAL,
            INPUT,
            OUTPUT,
        }

        impl SRC {
            pub fn to_label(&self) -> String {
                match self {
                    SRC::GENERAL => String::from("both"),
                    SRC::INPUT => String::from("input"),
                    SRC::OUTPUT => String::from("output"),
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
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    INPUT => Ok(SRC::INPUT),
                    OUTPUT => Ok(SRC::OUTPUT),
                    GENERAL => Ok(SRC::GENERAL),
                    _ => Err(()),
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
}
// STATUS CODES RETURNING FROM MATRIX
pub mod status_codes {
    pub const SUCCESS: &str = "00";
    pub const ERR: &str = "01";
}
