use std::str::FromStr;

use backend::audio_engine::defs::{errors::Error,datas::{io::*, rw}};
use backend::audio_engine::{mute::*, presets::read_current_preset, volume::*};
use backend::audio_engine::lib::*;
use backend::services::private::app::schemas::SetState;

#[test]
fn ok_read_mute_cmd() {
    let cmd = read_mute_ch(SRC::INPUT, 16).unwrap();
    assert_eq!(
        cmd.to_string(),
        "A5 C3 3C 5A FF 63 03 02 01 16 EE".to_string()
    )
}

#[test]
fn ok_read_mute_all_cmd() {
    let cmds = read_mute_all(SRC::INPUT).unwrap();
    let mut string = String::new();
    for cmd in cmds {
        string.push_str(cmd.to_string().as_str());
        string.push('\n');
    }
    assert_eq!(
        string.trim(),
        "A5 C3 3C 5A FF 63 03 02 01 01 EE
A5 C3 3C 5A FF 63 03 02 01 02 EE
A5 C3 3C 5A FF 63 03 02 01 03 EE
A5 C3 3C 5A FF 63 03 02 01 04 EE
A5 C3 3C 5A FF 63 03 02 01 05 EE
A5 C3 3C 5A FF 63 03 02 01 06 EE
A5 C3 3C 5A FF 63 03 02 01 07 EE
A5 C3 3C 5A FF 63 03 02 01 08 EE
A5 C3 3C 5A FF 63 03 02 01 09 EE
A5 C3 3C 5A FF 63 03 02 01 10 EE
A5 C3 3C 5A FF 63 03 02 01 11 EE
A5 C3 3C 5A FF 63 03 02 01 12 EE
A5 C3 3C 5A FF 63 03 02 01 13 EE
A5 C3 3C 5A FF 63 03 02 01 14 EE
A5 C3 3C 5A FF 63 03 02 01 15 EE
A5 C3 3C 5A FF 63 03 02 01 16 EE"
    )
}

#[test]
fn err_read_mute_cmd_over() {
    let cmd = read_mute_ch(SRC::OUTPUT, 43);

    assert_eq!(cmd.unwrap_err().to_string(),Error::InvalidChannel.to_string());

    let cmd = read_mute_ch(SRC::INPUT, 17);

    assert_eq!(cmd.unwrap_err().to_string(), Error::InvalidChannel.to_string());
}


#[test]
fn err_read_volume_cmd_over() {
    let cmd = read_volume_ch(SRC::OUTPUT, 43);

    assert_eq!(cmd.unwrap_err().to_string(), Error::InvalidChannel.to_string());

    let cmd = read_volume_ch(SRC::INPUT, 17);

    assert_eq!(cmd.unwrap_err().to_string(), Error::InvalidChannel.to_string());
}

#[test]
fn err_read_mute_cmd_under() {
    let cmd = read_mute_ch(SRC::INPUT, 0);

    assert_eq!(cmd.unwrap_err().to_string(), Error::InvalidChannel.to_string());

    let cmd = read_mute_ch(SRC::OUTPUT, 0);

    assert_eq!(cmd.unwrap_err().to_string(), Error::InvalidChannel.to_string())
}

#[test]
fn ok_read_volume_cmd() {
    let cmd = read_volume_ch(SRC::INPUT, 5).unwrap();

    assert_eq!(
        cmd.to_string(),
        "A5 C3 3C 5A FF 63 04 02 01 05 EE".to_string()
    )
}

#[test]
fn ok_read_volume_all_cmd() {
    let cmds = read_volume_all(SRC::INPUT).unwrap();
    let mut string = String::new();
    for cmd in cmds {
        string.push_str(cmd.to_string().as_str());
        string.push('\n');
    }
    assert_eq!(
        string.trim(),
        "A5 C3 3C 5A FF 63 04 02 01 01 EE
A5 C3 3C 5A FF 63 04 02 01 02 EE
A5 C3 3C 5A FF 63 04 02 01 03 EE
A5 C3 3C 5A FF 63 04 02 01 04 EE
A5 C3 3C 5A FF 63 04 02 01 05 EE
A5 C3 3C 5A FF 63 04 02 01 06 EE
A5 C3 3C 5A FF 63 04 02 01 07 EE
A5 C3 3C 5A FF 63 04 02 01 08 EE
A5 C3 3C 5A FF 63 04 02 01 09 EE
A5 C3 3C 5A FF 63 04 02 01 10 EE
A5 C3 3C 5A FF 63 04 02 01 11 EE
A5 C3 3C 5A FF 63 04 02 01 12 EE
A5 C3 3C 5A FF 63 04 02 01 13 EE
A5 C3 3C 5A FF 63 04 02 01 14 EE
A5 C3 3C 5A FF 63 04 02 01 15 EE
A5 C3 3C 5A FF 63 04 02 01 16 EE"
    )
}

#[test]
fn ok_read_current_preset_cmd() {
    let cmd = read_current_preset().unwrap();
    assert_eq!(cmd.to_string(), "A5 C3 3C 5A FF 63 02 00 EE".to_string())
}


#[test]
fn ok_read_all_states(){
    let cmds = read_all_states().unwrap();
    let mut string = String::new();
    for cmd in cmds {
        string.push_str(cmd.to_string().as_str());
        string.push('\n');
    }
    assert_eq!(string.trim(),
    "A5 C3 3C 5A FF 63 03 02 01 01 EE
A5 C3 3C 5A FF 63 03 02 01 02 EE
A5 C3 3C 5A FF 63 03 02 01 03 EE
A5 C3 3C 5A FF 63 03 02 01 04 EE
A5 C3 3C 5A FF 63 03 02 01 05 EE
A5 C3 3C 5A FF 63 03 02 01 06 EE
A5 C3 3C 5A FF 63 03 02 01 07 EE
A5 C3 3C 5A FF 63 03 02 01 08 EE
A5 C3 3C 5A FF 63 03 02 01 09 EE
A5 C3 3C 5A FF 63 03 02 01 10 EE
A5 C3 3C 5A FF 63 03 02 01 11 EE
A5 C3 3C 5A FF 63 03 02 01 12 EE
A5 C3 3C 5A FF 63 03 02 01 13 EE
A5 C3 3C 5A FF 63 03 02 01 14 EE
A5 C3 3C 5A FF 63 03 02 01 15 EE
A5 C3 3C 5A FF 63 03 02 01 16 EE
A5 C3 3C 5A FF 63 03 02 02 01 EE
A5 C3 3C 5A FF 63 03 02 02 02 EE
A5 C3 3C 5A FF 63 03 02 02 03 EE
A5 C3 3C 5A FF 63 03 02 02 04 EE
A5 C3 3C 5A FF 63 03 02 02 05 EE
A5 C3 3C 5A FF 63 03 02 02 06 EE
A5 C3 3C 5A FF 63 03 02 02 07 EE
A5 C3 3C 5A FF 63 03 02 02 08 EE
A5 C3 3C 5A FF 63 03 02 02 09 EE
A5 C3 3C 5A FF 63 03 02 02 10 EE
A5 C3 3C 5A FF 63 03 02 02 11 EE
A5 C3 3C 5A FF 63 03 02 02 12 EE
A5 C3 3C 5A FF 63 03 02 02 13 EE
A5 C3 3C 5A FF 63 03 02 02 14 EE
A5 C3 3C 5A FF 63 03 02 02 15 EE
A5 C3 3C 5A FF 63 03 02 02 16 EE
A5 C3 3C 5A FF 63 04 02 01 01 EE
A5 C3 3C 5A FF 63 04 02 01 02 EE
A5 C3 3C 5A FF 63 04 02 01 03 EE
A5 C3 3C 5A FF 63 04 02 01 04 EE
A5 C3 3C 5A FF 63 04 02 01 05 EE
A5 C3 3C 5A FF 63 04 02 01 06 EE
A5 C3 3C 5A FF 63 04 02 01 07 EE
A5 C3 3C 5A FF 63 04 02 01 08 EE
A5 C3 3C 5A FF 63 04 02 01 09 EE
A5 C3 3C 5A FF 63 04 02 01 10 EE
A5 C3 3C 5A FF 63 04 02 01 11 EE
A5 C3 3C 5A FF 63 04 02 01 12 EE
A5 C3 3C 5A FF 63 04 02 01 13 EE
A5 C3 3C 5A FF 63 04 02 01 14 EE
A5 C3 3C 5A FF 63 04 02 01 15 EE
A5 C3 3C 5A FF 63 04 02 01 16 EE
A5 C3 3C 5A FF 63 04 02 02 01 EE
A5 C3 3C 5A FF 63 04 02 02 02 EE
A5 C3 3C 5A FF 63 04 02 02 03 EE
A5 C3 3C 5A FF 63 04 02 02 04 EE
A5 C3 3C 5A FF 63 04 02 02 05 EE
A5 C3 3C 5A FF 63 04 02 02 06 EE
A5 C3 3C 5A FF 63 04 02 02 07 EE
A5 C3 3C 5A FF 63 04 02 02 08 EE
A5 C3 3C 5A FF 63 04 02 02 09 EE
A5 C3 3C 5A FF 63 04 02 02 10 EE
A5 C3 3C 5A FF 63 04 02 02 11 EE
A5 C3 3C 5A FF 63 04 02 02 12 EE
A5 C3 3C 5A FF 63 04 02 02 13 EE
A5 C3 3C 5A FF 63 04 02 02 14 EE
A5 C3 3C 5A FF 63 04 02 02 15 EE
A5 C3 3C 5A FF 63 04 02 02 16 EE
A5 C3 3C 5A FF 63 02 00 EE"
)
}


#[test]
fn ok_from_str_to_matrix_command(){
    let cmd = MatrixCommand::from_str("A5 C3 3C 5A FF 63 03 02 02 16 EE").unwrap();
    
    assert_eq!(cmd, read_mute_ch(SRC::OUTPUT, 16).unwrap());

    let cmd = MatrixCommand::from_str("A5 C3 3C 5A FF 63 02 00 EE").unwrap();
    
    assert_eq!(cmd,read_current_preset().unwrap());
}

#[test]

fn ok_from_bytes_to_matrix_command(){
    let bytes = read_mute_ch(SRC::OUTPUT, 16).unwrap().to_byte_hex().unwrap();
    assert_eq!(read_mute_ch(SRC::OUTPUT, 16).unwrap(), MatrixCommand::try_from(&bytes[..]).unwrap())
}


#[test]
fn ok_from_bytes_to_matrix_status_code(){
    let bytes = "00".as_bytes();
    let res = MatrixReturnCode::try_from(bytes);
    assert!(res.is_ok());
    let bytes = "01".as_bytes();
    let res = MatrixReturnCode::try_from(bytes);
    assert!(res.is_ok());
    let bytes = "02".as_bytes();
    let res = MatrixReturnCode::try_from(bytes);
    assert!(res.is_err()); 

}

#[test]
fn err_conversion_error_for_status_code(){
    let bytes = "00".as_bytes();
    let res = MatrixCommand::try_from(bytes);
    assert_eq!(res.unwrap_err().to_string(),"Invalid format, no start code found".to_string());
}

#[test]
fn parse_preset_to_cmd(){
    let raw_cmd = "A5 C3 3C 5A FF 63 02 00 EE";
    let bytes = MatrixCommand::from_str(raw_cmd).unwrap().to_byte_hex().unwrap();
    MatrixCommand::try_from(&bytes[..]).unwrap();
}

#[test]
fn ok_cmd_from_wsclient_simulation(){
    let set_states = SetState{
        section: "volume".to_string(),
        io:Some("output".to_string()),
        channel: Some("16".to_string()),
        value:Some("-60.0".to_string())
    };

    let cmd = MatrixCommand::new_from_client(rw::WRITE.to_string(),set_states);
    assert!(cmd.is_ok());
    println!("{}",cmd.clone().unwrap().to_string());
    assert_eq!(cmd.unwrap().to_string(),"A5 C3 3C 5A FF 36 04 04 02 16 A8 FD EE".to_string())
}