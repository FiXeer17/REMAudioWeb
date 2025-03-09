
use crate::engine::{defs::datas::io::SRC, lib::mute::{read_mute_all, read_mute_ch}};


#[test]
fn ok_read_mute_cmd(){
    let cmd = read_mute_ch(SRC::INPUT, 16).unwrap();
    assert_eq!(cmd.to_string(), "A5 C3 3C 5A FF 63 03 02 01 16 EE".to_string())
}


#[test]
fn ok_read_mute_all_cmd(){
    let cmds = read_mute_all(SRC::INPUT).unwrap();
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
A5 C3 3C 5A FF 63 03 02 01 16 EE")
}


#[test]
fn err_read_mute_cmd_over() {
    let cmd = read_mute_ch(SRC::OUTPUT, 43);

    assert_eq!(cmd.unwrap_err().to_string(), "Invalid channel".to_string());

    let cmd = read_mute_ch(SRC::INPUT, 17);

    assert_eq!(cmd.unwrap_err().to_string(), "Invalid channel".to_string());
}

#[test]
fn err_read_mute_cmd_under(){
    let cmd = read_mute_ch(SRC::INPUT, 0);

    assert_eq!(cmd.unwrap_err().to_string(), "Invalid channel".to_string());

    let cmd = read_mute_ch(SRC::OUTPUT, 0);

    assert_eq!(cmd.unwrap_err().to_string(), "Invalid channel".to_string())
}
