use std::collections::HashMap;
use std::net::SocketAddrV4;
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::configs::ping_socket_settings;

pub const AUDIO_LABEL:&str="matrix";
pub const VIDEO_LABEL:&str="camera";

#[derive(Debug,Clone)]
pub enum Device{
    Audio,
    Video
}

impl ToString for Device{
    fn to_string(&self) -> String {
        match self{
            Self::Audio => AUDIO_LABEL.to_string(),
            Self::Video => VIDEO_LABEL.to_string(),
        }
    }
}
impl FromStr for Device{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            AUDIO_LABEL => Ok(Self::Audio),
            VIDEO_LABEL => Ok(Self::Video),
            _ => Err(())
        }
    }
}
pub async fn try_connection(socket: SocketAddrV4) -> bool {
    let max_retries = ping_socket_settings::get_ping_socket_max_retries();
    let timeout_interval = ping_socket_settings::get_ping_socket_timeout();
    let mut i=0;
    while i < max_retries {
        match timeout(timeout_interval,TcpStream::connect(socket)).await {
            Ok(not_timedout) => {
                if let Ok(tcp_stream) = not_timedout{
                    drop(tcp_stream);
                    return true;
                }else{
                    return false;
                }
            }
            Err(_) => {
                i+=1;
            }
        }
    }
    return false;
}

pub fn check_in_connections(socket:SocketAddrV4,connections:Option<HashMap<SocketAddrV4,String>>)-> bool{
    match connections{
        Some(connections)=>{
            connections.keys().cloned().collect::<Vec<SocketAddrV4>>().contains(&socket)
        },
        None => false
    }
}
