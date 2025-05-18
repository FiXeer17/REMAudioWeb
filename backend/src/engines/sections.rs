use std::str::FromStr;

    use super::video_engine::defs::fncodes::{MOVE_CAMERA_LABEL, PRESETS_LABEL, ZOOM_TELE_LABEL, ZOOM_WIDE_LABEL,ZOOM_STOP_LABEL};
    use super::{audio_engine, video_engine};
    use super::audio_engine::defs::{errors, fncodes::*};


    pub const VISIBILITY_LABEL :&str = "visibility";
    pub const CHANNEL_LABELS_LABEL : &str = "channel_labels";
    pub const MATRIX_PRESET_LABELS_LABEL : &str = "matrix_preset_labels";
    pub const CAMERA_PRESET_LABELS_LABEL : &str = "camera_preset_labels";


    pub enum Sections{
        Visibility,
        ChannelLabels,
        MatrixPresetLabels,
        CameraPresetLabels,
        MatrixCommand(audio_engine::defs::fncodes::FNCODE),
        CameraCommand(video_engine::defs::fncodes::FNCODE)
    }

    impl ToString for Sections{
        fn to_string(&self) -> String {
            match self{
                Sections::ChannelLabels => String::from(CHANNEL_LABELS_LABEL),
                Sections::MatrixPresetLabels => String::from(MATRIX_PRESET_LABELS_LABEL),
                Sections::CameraPresetLabels => String::from(CAMERA_PRESET_LABELS_LABEL),
                Sections::Visibility => String::from(VISIBILITY_LABEL),
                Sections::MatrixCommand(cmd) => cmd.to_label(),
                Sections::CameraCommand(cmd) => cmd.to_label(),
            }
        }
    }
    impl FromStr for Sections {
        type Err = errors::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                VISIBILITY_LABEL => Ok(Sections::Visibility),
                CHANNEL_LABELS_LABEL => Ok(Sections::ChannelLabels),
                MATRIX_PRESET_LABELS_LABEL => Ok(Sections::MatrixPresetLabels),
                CAMERA_PRESET_LABELS_LABEL => Ok(Sections::CameraPresetLabels),
                VOLUME_LABEL => Ok(Sections::MatrixCommand(audio_engine::defs::fncodes::FNCODE::VOLUME)),
                MUTE_LABEL => Ok(Sections::MatrixCommand(audio_engine::defs::fncodes::FNCODE::MUTE)),
                SCENE_LABEL => Ok(Sections::MatrixCommand(audio_engine::defs::fncodes::FNCODE::SCENE)),
                MATRIX_MIXING_LABEL => Ok(Sections::MatrixCommand(audio_engine::defs::fncodes::FNCODE::MATRIXMIXING)),
                PRESETS_LABEL => Ok(Sections::CameraCommand(video_engine::defs::fncodes::FNCODE::Preset)),
                ZOOM_TELE_LABEL => Ok(Sections::CameraCommand(video_engine::defs::fncodes::FNCODE::ZoomTele)),
                ZOOM_WIDE_LABEL => Ok(Sections::CameraCommand(video_engine::defs::fncodes::FNCODE::ZoomWide)),
                ZOOM_STOP_LABEL => Ok(Sections::CameraCommand(video_engine::defs::fncodes::FNCODE::ZoomStop)),
                MOVE_CAMERA_LABEL => Ok(Sections::CameraCommand(video_engine::defs::fncodes::FNCODE::MoveCamera)),
                _ => Err(errors::Error::InvalidSection)
            }
        }
    }
