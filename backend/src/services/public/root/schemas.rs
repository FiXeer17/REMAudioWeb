use std::net::SocketAddrV4;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IpPortCouple {
    pub ip: String,
    pub port: u16,
}
#[derive(Deserialize, Serialize)]
pub struct ReturnSockets {
    pub sockets: Option<Vec<IpPortCouple>>,
}
impl ReturnSockets {
    pub fn new(sockets: Option<Vec<SocketAddrV4>>) -> Self {
        ReturnSockets {
            sockets: sockets.map(|sockets| {
                sockets
                    .into_iter()
                    .map(|socket| IpPortCouple {
                        ip: socket.ip().to_string(),
                        port: socket.port(),
                    })
                    .collect()
            }),
        }
    }
}
