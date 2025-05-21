
use backend::engines::video_engine::defs;
use backend::engines::video_engine::camera_presets_lib;


#[test]
fn call_preset_ok(){
    let preset_min = camera_presets_lib::call_preset("0x1".to_string()).unwrap();
    assert_eq!(preset_min,[0x81,0x01,0x04,0x3F,0x02,0x01,0xff]);
    let preset_max = camera_presets_lib::call_preset("0x09".to_string()).unwrap();
    assert_eq!(preset_max,[0x81,0x01,0x04,0x3F,0x02,0x09,0xff]);
    let preset_should_fail = camera_presets_lib::call_preset("0x0A".to_string());
    assert_eq!(preset_should_fail.unwrap_err(),defs::errors::Error::InvalidPreset);
}

