use std::str::FromStr;

use crate::{
    engines::{
        audio_engine::{defs::datas, lib::MatrixCommand},
        sections::Sections,
        video_engine::{
            camera_presets_lib::call_preset,
            defs::{
                camera_zoom::{ZOOM_STOP, ZOOM_TELE, ZOOM_WIDE},
                fncodes::FNCODE,
                pan_tilt::Direction,
                CameraCommand,
            },
            tilt_pan_lib::{move_camera, return_home},
        },
    },
    services::private::{
        app::schemas::{SetAttributes, SetState},
        socket::utils::Device,
    },
};

use super::utils::HandleText;

pub fn handle_visibility(set_state: SetState) -> HandleText {
    if set_state.io.is_none() {
        return HandleText::Error("io is none".to_string());
    };
    let Some(ch) = set_state.channel.clone() else {
        return HandleText::Error("channel is none".to_string());
    };
    let Some(value) = set_state.value.clone() else {
        return HandleText::Error("value is none".to_string());
    };
    let Ok(_) = ch.parse::<u32>() else {
        return HandleText::Error("channel is invalid".to_string());
    };
    let Ok(_) = value.parse::<bool>() else {
        return HandleText::Error("value is invalid".to_string());
    };

    let set_visibility = SetAttributes {
        io: set_state.io,
        channel: set_state.channel,
        value: set_state.value.unwrap(),
        index: None,
        device: None,
    };
    HandleText::SetVisibility(set_visibility)
}

//fn handle_label(set_state: SetState) -> HandleText{}
pub fn handle_matrix_command(set_state: SetState) -> HandleText {
    let rw = datas::rw::WRITE.to_string();
    HandleText::MatrixCommand(MatrixCommand::new_from_client(rw, set_state))
}

pub fn handle_channel_label(set_state: SetState) -> HandleText {
    let Some(_) = set_state.io else {
        return HandleText::Error("io is none".to_string());
    };
    let Some(ch) = set_state.channel.clone() else {
        return HandleText::Error("channel is none".to_string());
    };
    let Some(value) = set_state.value.clone() else {
        return HandleText::Error("value is none".to_string());
    };
    let Ok(_) = ch.parse::<u32>() else {
        return HandleText::Error("cannot parse channel".to_string());
    };
    HandleText::SetChannelLabels(SetAttributes {
        io: set_state.io,
        channel: set_state.channel,
        value,
        index: None,
        device: None,
    })
}

pub fn handle_video_command(set_state: SetState, section: &Sections) -> HandleText {
    match section {
        Sections::CameraCommand(sc) => match sc {
            FNCODE::Preset => {
                let call_preset = call_preset(set_state.value.unwrap());
                if let Err(e) = call_preset {
                    return HandleText::CameraCommand(Err(e));
                } else {
                    return HandleText::CameraCommand(Ok(CameraCommand {
                        fncode: sc.clone(),
                        cmd: call_preset.unwrap(),
                    }));
                }
            }
            FNCODE::ZoomTele => {
                let zoom_tele = ZOOM_TELE.to_vec();
                return HandleText::CameraCommand(Ok(CameraCommand {
                    fncode: sc.clone(),
                    cmd: zoom_tele,
                }));
            }
            FNCODE::ZoomWide => {
                let zoom_wide = ZOOM_WIDE.to_vec();
                return HandleText::CameraCommand(Ok(CameraCommand {
                    fncode: sc.clone(),
                    cmd: zoom_wide,
                }));
            }
            FNCODE::ZoomStop => {
                let zoom_stop = ZOOM_STOP.to_vec();
                return HandleText::CameraCommand(Ok(CameraCommand {
                    fncode: sc.clone(),
                    cmd: zoom_stop,
                }));
            }
            FNCODE::MoveCamera => {
                let Some(direction) = set_state.direction else {
                    return HandleText::Error("Direction not found".to_string());
                };
                let Ok(direction_e) = Direction::from_str(&direction) else {
                    return HandleText::Error("Invalid direction found".to_string());
                };
                if set_state.velocity.is_none() {
                    if direction_e == Direction::HOME {
                        return HandleText::CameraCommand(Ok(CameraCommand {
                            fncode: sc.clone(),
                            cmd: return_home(),
                        }));
                    } else {
                        return HandleText::Error("Velocity not found".to_string());
                    }
                }

                let move_camera = move_camera(set_state.velocity.unwrap(), direction);
                if let Err(e) = move_camera {
                    return HandleText::CameraCommand(Err(e));
                }
                HandleText::CameraCommand(Ok(CameraCommand {
                    fncode: sc.clone(),
                    cmd: move_camera.unwrap(),
                }))
            }
            FNCODE::ReadPreset => unreachable!(),
        },
        _ => HandleText::Error("Invalid video command".to_string()),
    }
}

pub fn handle_preset_label(set_state: SetState) -> HandleText {
    let Some(index) = set_state.index.clone() else {
        return HandleText::Error("index is none".to_string());
    };
    let Some(value) = set_state.value.clone() else {
        return HandleText::Error("value is none".to_string());
    };
    let Ok(_) = index.parse::<u32>() else {
        return HandleText::Error("cannot parse index".to_string());
    };
    let section = Sections::from_str(&set_state.section).unwrap();
    match section {
        Sections::MatrixPresetLabels => HandleText::SetPresetLabels(SetAttributes {
            io: None,
            index: set_state.index,
            value,
            channel: None,
            device: Some(Device::Audio),
        }),
        Sections::CameraPresetLabels => HandleText::SetPresetLabels(SetAttributes {
            io: None,
            index: set_state.index,
            value,
            channel: None,
            device: Some(Device::Video),
        }),
        _ => HandleText::Error("Invalid section".to_string()),
    }
}
