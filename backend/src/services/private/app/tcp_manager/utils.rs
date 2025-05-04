use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::SocketAddrV4,
    str::FromStr,
};

use actix::Addr;
use log::info;

use crate::{
    services::{
        private::{
            app::{
                messages::{InactiveQueue, SocketRestarted, UnavailableSockets},
                schemas::MatrixStates,
                ws_session::session::WsSession,
            },
            socket::utils::{try_connection, Device},
        },
        public::{interfaces::{is_socket_in_db, retrieve_sockets}, schemas::Socket},
    },
    AppState,
};

use super::tcp_manager::TcpStreamsManager;

pub fn attach_availability(
    mut states: MatrixStates,
    availability: &Option<Addr<WsSession>>,
    session: &Addr<WsSession>,
) -> MatrixStates {
    if let Some(wsocket) = availability {
        if wsocket != session {
            states.available = Some(false)
        } else {
            states.available = Some(true)
        }
    } else {
        states.available = Some(true)
    }
    states
}

pub async fn load_sockets_from_db(
    pgpool: actix_web::web::Data<AppState>,
) -> Result<(HashMap<Socket, String>, Option<SocketAddrV4>,Option<SocketAddrV4>), sqlx::Error> {
    let sockets = retrieve_sockets(&pgpool).await?;
    let mut map: HashMap<Socket, String> = HashMap::new();
    sockets.iter().for_each(|socket| {
        map.insert(socket.clone(), socket.socket_name.clone());
    });
    let latest_audio_socket = sockets.iter().find_map(|sock| {
        if sock.latest && sock.device == Device::Audio.to_string() {
            return Some(SocketAddrV4::from_str(&sock.socket).unwrap());
        }
        None
    });
    let latest_video_socket = sockets.iter().find_map(|sock| {
        if sock.latest && sock.device == Device::Video.to_string() {
            return Some(SocketAddrV4::from_str(&sock.socket).unwrap());
        }
        None
    });
    return Ok((map, latest_audio_socket,latest_video_socket));
}

pub async fn remove_inactive_connection(
    pgpool: actix_web::web::Data<AppState>,
) -> Result<(HashSet<Socket>, Option<SocketAddrV4>, Option<SocketAddrV4>), sqlx::Error> {
    let (mut sockets, mut latest_audio_socket,mut latest_video_socket) = load_sockets_from_db(pgpool.clone()).await?;
    let mut inactive_sockets: Vec<Socket> = Vec::new();
    for socket in sockets.keys() {
        let sockv4 = SocketAddrV4::from_str(&socket.socket).unwrap();
        if !try_connection(sockv4).await {
            inactive_sockets.push(socket.clone());
        }
    }
    for socket in inactive_sockets {
        info!(
            "Inactive connection found, removing socket: {}...",
            socket.socket_name
        );
        sockets.remove(&socket).unwrap();
    }

    if let Some(latest) = latest_audio_socket {
        if !try_connection(latest).await {
            info!(
                "Inactive connection found, removing latest_socket: {}...",
                latest.to_string()
            );
            latest_audio_socket = None;
        }
    }
    if let Some(latest) = latest_video_socket {
        if !try_connection(latest).await {
            info!(
                "Inactive connection found, removing latest_socket: {}...",
                latest.to_string()
            );
            latest_video_socket = None;
        }
    }
    let sockets:HashSet<Socket> = sockets.keys().cloned().collect();
    Ok((sockets, latest_audio_socket,latest_video_socket))
}

pub async fn detect_dead_sockets(socks: Vec<Socket>) -> Result<Vec<Socket>, sqlx::Error> {
    let mut inactive_sockets: Vec<Socket> = Vec::new();
    for socket in socks {
        if !try_connection(SocketAddrV4::from_str(&socket.socket).unwrap()).await {
            inactive_sockets.push(socket);
        }
    }

    Ok(inactive_sockets)
}
impl TcpStreamsManager {
    pub fn poll_sockets(
        mut inactive_sockets: VecDeque<Socket>,
        addr: Addr<Self>,
        pgpool: actix_web::web::Data<AppState>,
    ) {
        tokio::spawn(async move {
            if !inactive_sockets.is_empty() {
                let to_test = inactive_sockets.pop_back().unwrap();
                if let Ok(res) = is_socket_in_db(&pgpool, SocketAddrV4::from_str(&to_test.socket).unwrap()).await{
                    if !res {return;}
                }
                let response =
                    try_connection(SocketAddrV4::from_str(&to_test.socket).unwrap()).await;
                if response {
                    if to_test.latest {
                        addr.do_send(SocketRestarted {
                            latest_socket: Some(to_test),
                            socket: None,
                        });
                    } else {
                        addr.do_send(SocketRestarted {
                            latest_socket: None,
                            socket: Some(to_test),
                        })
                    }
                } else {
                    inactive_sockets.push_front(to_test);
                }
            }
            let sockets = retrieve_sockets(&pgpool).await;
            if let Ok(socks) = sockets {
                let inactive = detect_dead_sockets(socks).await;
                if let Ok(sockets) = inactive {
                    addr.do_send(UnavailableSockets {
                        sockets: sockets.clone(),
                    });
                    sockets.into_iter().for_each(|sock| {
                        if !inactive_sockets.contains(&sock) {
                            inactive_sockets.push_front(sock)
                        }
                    });
                }
            }
            addr.do_send(InactiveQueue {
                queue: inactive_sockets,
            });
        });
    }
}
