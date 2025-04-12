use std::{collections::HashMap, net::SocketAddrV4};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NameIpPort {
    pub name: String,
    pub ip: String,
    pub port: String,
}
#[derive(Deserialize, Serialize)]
pub struct ReturnSockets {
    pub sockets: Option<Vec<NameIpPort>>,
    pub latest_socket: Option<NameIpPort>,
}
impl ReturnSockets {
    pub fn new(
        sockets: Option<HashMap<SocketAddrV4, String>>,
        latest_socket: Option<HashMap<SocketAddrV4, String>>,
    ) -> Self {
        let mut sockets_without_latest = sockets;
        if sockets_without_latest.is_some() && sockets_without_latest.is_some() {
            sockets_without_latest.as_mut()
                .unwrap()
                .remove(latest_socket.clone().unwrap().iter().next().unwrap().0);
        }
        let latest_socket = latest_socket.as_ref().and_then(|map| {
            map.iter().next().map(|(socket, name)| NameIpPort {
                name: name.clone(),
                ip: socket.ip().to_string(),
                port: socket.port().to_string(),
            })
        });

        let sockets = sockets_without_latest.map(|map| {
            map.into_iter()
                .map(|(socket, name)| NameIpPort {
                    name,
                    ip: socket.ip().to_string(),
                    port: socket.port().to_string(),
                })
                .collect()
        });

        ReturnSockets {
            sockets,
            latest_socket,
        }
    }
}
