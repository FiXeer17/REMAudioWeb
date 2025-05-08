
use backend::engines::video_engine::defs;
use backend::engines::video_engine::defs::pan_tilt::Direction;
use backend::engines::video_engine::zoom_lib;
use backend::engines::video_engine::camera_presets_lib;
use backend::engines::video_engine::tilt_pan_lib;

#[test]
fn zoom_tele_ok(){
    let zoom_max = zoom_lib::zoom_tele(0xf).unwrap();
    assert_eq!(zoom_max,[0x08,0x01,0x04,0x07,0x2f,0xff]);
    let zoom_min = zoom_lib::zoom_tele(0).unwrap();
    assert_eq!(zoom_min,[0x08,0x01,0x04,0x07,0x20,0xff]);
    let zoom_should_fail = zoom_lib::zoom_tele(0x10);
    assert_eq!(zoom_should_fail.unwrap_err(),defs::camera_zoom::Error::InvalidCoefficient);

}

#[test]
fn zoom_wide_ok(){
    let zoom_max = zoom_lib::zoom_wide(0xf).unwrap();
    assert_eq!(zoom_max,[0x08,0x01,0x04,0x07,0x3f,0xff]);
    let zoom_min = zoom_lib::zoom_wide(0).unwrap();
    assert_eq!(zoom_min,[0x08,0x01,0x04,0x07,0x30,0xff]);
    let zoom_should_fail = zoom_lib::zoom_tele(0x10);
    assert_eq!(zoom_should_fail.unwrap_err(),defs::camera_zoom::Error::InvalidCoefficient);

}

#[test]
fn call_preset_ok(){
    let preset_min = camera_presets_lib::call_preset(0x1).unwrap();
    assert_eq!(preset_min,[0x08,0x01,0x04,0x3F,0x02,0x01,0xff]);
    let preset_max = camera_presets_lib::call_preset(0x09).unwrap();
    assert_eq!(preset_max,[0x08,0x01,0x04,0x3F,0x02,0x09,0xff]);
    let preset_should_fail = camera_presets_lib::call_preset(0x0A);
    assert_eq!(preset_should_fail.unwrap_err(),defs::camera_presets::Error::InvalidPreset);
}


#[test]
fn move_camera(){
    let move_up_min_vel = tilt_pan_lib::move_camera(1, 1, Direction::UP).unwrap();
    let move_down_min_vel = tilt_pan_lib::move_camera(1, 1, Direction::DOWN).unwrap();
    let move_left_min_vel = tilt_pan_lib::move_camera(1,1,Direction::LEFT).unwrap();
    let move_right_min_vel = tilt_pan_lib::move_camera(1,0x1,Direction::RIGHT).unwrap();
    assert_eq!(move_up_min_vel,[0x08,0x01,0x06,0x01,0x01,0x01,0x03,0x01,0xff]);
    assert_eq!(move_down_min_vel,[0x08,0x01,0x06,0x01,0x01,0x01,0x03,0x02,0xff]);
    assert_eq!(move_left_min_vel,[0x08,0x01,0x06,0x01,0x01,0x01,0x01,0x03,0xff]);
    assert_eq!(move_right_min_vel,[0x08,0x01,0x06,0x01,0x01,0x01,0x02,0x03,0xff]);

    let move_up_max_vel = tilt_pan_lib::move_camera(0x18, 0x14, Direction::UP).unwrap();
    let move_down_max_vel = tilt_pan_lib::move_camera(24, 20, Direction::DOWN).unwrap();
    let move_left_max_vel = tilt_pan_lib::move_camera(24,20,Direction::LEFT).unwrap();
    let move_right_max_vel = tilt_pan_lib::move_camera(24,20,Direction::RIGHT).unwrap();
    assert_eq!(move_up_max_vel,[0x08,0x01,0x06,0x01,24,0x14,0x03,0x01,0xff]);
    assert_eq!(move_down_max_vel,[0x08,0x01,0x06,0x01,24,0x14,0x03,0x02,0xff]);
    assert_eq!(move_left_max_vel,[0x08,0x01,0x06,0x01,24,0x14,0x01,0x03,0xff]);
    assert_eq!(move_right_max_vel,[0x08,0x01,0x06,0x01,24,0x14,0x02,0x03,0xff]);

    let move_up_min_vel_should_fail = tilt_pan_lib::move_camera(0, 0, Direction::UP);
    let move_down_min_vel_should_fail = tilt_pan_lib::move_camera(0x1, 0, Direction::DOWN);
    let move_left_min_vel_should_fail = tilt_pan_lib::move_camera(0,0,Direction::LEFT);
    let move_right_min_vel_should_fail = tilt_pan_lib::move_camera(0,0x1,Direction::RIGHT);
    assert_eq!(move_up_min_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidPan);
    assert_eq!(move_down_min_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidTilt);
    assert_eq!(move_left_min_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidPan);
    assert_eq!(move_right_min_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidPan);

    
    let move_up_max_vel_should_fail = tilt_pan_lib::move_camera(0x19, 0x14, Direction::UP);
    let move_down_max_vel_should_fail = tilt_pan_lib::move_camera(25, 20, Direction::DOWN);
    let move_left_max_vel_should_fail = tilt_pan_lib::move_camera(24,21,Direction::LEFT);
    let move_right_max_vel_should_fail = tilt_pan_lib::move_camera(24,0xff,Direction::RIGHT);
    assert_eq!(move_up_max_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidPan);
    assert_eq!(move_down_max_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidPan);
    assert_eq!(move_left_max_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidTilt);
    assert_eq!(move_right_max_vel_should_fail.unwrap_err(),defs::pan_tilt::Error::InvalidTilt);

}

