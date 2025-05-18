
use backend::engines::video_engine::defs;
use backend::engines::video_engine::zoom_lib;
use backend::engines::video_engine::camera_presets_lib;

#[test]
fn zoom_tele_ok(){
    let zoom_max = zoom_lib::zoom_tele("0xf".to_string()).unwrap();
    assert_eq!(zoom_max,[0x81,0x01,0x04,0x07,0x2f,0xff]);
    let zoom_min = zoom_lib::zoom_tele("0".to_string()).unwrap();
    assert_eq!(zoom_min,[0x081,0x01,0x04,0x07,0x20,0xff]);
    let zoom_should_fail = zoom_lib::zoom_tele("0x10".to_string());
    assert_eq!(zoom_should_fail.unwrap_err(),defs::errors::Error::InvalidCoefficient);

}

#[test]
fn zoom_wide_ok(){
    let zoom_max = zoom_lib::zoom_wide("0xf".to_string()).unwrap();
    assert_eq!(zoom_max,[0x81,0x01,0x04,0x07,0x3f,0xff]);
    let zoom_min = zoom_lib::zoom_wide("0".to_string()).unwrap();
    assert_eq!(zoom_min,[0x81,0x01,0x04,0x07,0x30,0xff]);
    let zoom_should_fail = zoom_lib::zoom_tele("0x10".to_string());
    assert_eq!(zoom_should_fail.unwrap_err(),defs::errors::Error::InvalidCoefficient);

}

#[test]
fn call_preset_ok(){
    let preset_min = camera_presets_lib::call_preset("0x1".to_string()).unwrap();
    assert_eq!(preset_min,[0x81,0x01,0x04,0x3F,0x02,0x01,0xff]);
    let preset_max = camera_presets_lib::call_preset("0x09".to_string()).unwrap();
    assert_eq!(preset_max,[0x81,0x01,0x04,0x3F,0x02,0x09,0xff]);
    let preset_should_fail = camera_presets_lib::call_preset("0x0A".to_string());
    assert_eq!(preset_should_fail.unwrap_err(),defs::errors::Error::InvalidPreset);
}

