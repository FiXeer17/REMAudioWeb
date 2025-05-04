use std::{
    collections::HashSet,
    net::SocketAddrV4,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::services::{
    private::socket::utils::Device,
    public::schemas::{IsContainedExt, Socket},
};

#[derive(Deserialize, Serialize)]
pub struct NameIpPort {
    pub name: String,
    pub ip: String,
    pub port: String,
    pub device_type: String,
}
#[derive(Deserialize, Serialize)]
pub struct ReturnSockets {
    pub sockets: Option<Vec<NameIpPort>>,
    pub latest_audio_socket: Option<NameIpPort>,
    pub latest_video_socket: Option<NameIpPort>,
}
impl ReturnSockets {
    pub fn new(sockets: Option<HashSet<Socket>>) -> Self {
        let (mut latest_audio_socket, mut latest_video_socket): (Option<Socket>, Option<Socket>) =
            (None, None);
        let sockets = if let Some(mut socket_set) = sockets {
            while let Some(latest) = socket_set.latest_is_contained() {
                socket_set.remove(&latest);
                let latest_dev = Device::from_str(&latest.device).unwrap();
                match latest_dev {
                    Device::Video => latest_video_socket = Some(latest),
                    Device::Audio => latest_audio_socket = Some(latest),
                }
            }

            if socket_set.is_empty() {
                None
            } else {
                Some(socket_set)
            }
        } else {
            None
        };

        let latest_audio_socket = latest_audio_socket.map(|sock| {
            let socket = SocketAddrV4::from_str(&sock.socket).unwrap();
            NameIpPort {
                name: sock.socket_name,
                ip: socket.ip().to_string(),
                port: socket.port().to_string(),
                device_type: sock.device,
            }
        });
        let latest_video_socket = latest_video_socket.map(|sock| {
            let socket = SocketAddrV4::from_str(&sock.socket).unwrap();
            NameIpPort {
                name: sock.socket_name,
                ip: socket.ip().to_string(),
                port: socket.port().to_string(),
                device_type: sock.device,
            }
        });

        let sockets = sockets.map(|map| {
            map.into_iter()
                .map(|sock| {
                    let socket = SocketAddrV4::from_str(&sock.socket).unwrap();
                    NameIpPort {
                        name: sock.socket_name,
                        ip: socket.ip().to_string(),
                        port: socket.port().to_string(),
                        device_type: sock.device,
                    }
                })
                .collect()
        });

        ReturnSockets {
            sockets,
            latest_audio_socket,
            latest_video_socket,
        }
    }
}
