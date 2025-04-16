use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddrV4,
    str::FromStr,
};

use actix::Addr;

use crate::{
    services::{
        private::{
            app::{
                messages::{InactiveQueue, SocketRestarted, UnavailableSockets},
                schemas::MatrixStates,
                ws_session::session::WsSession,
            },
            socket::utils::try_connection,
        },
        public::{interfaces::{retrieve_socket_from_db, retrieve_sockets}, schemas::Socket},
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
) -> Result<(HashMap<SocketAddrV4, String>, Option<SocketAddrV4>), sqlx::Error> {
    let sockets = retrieve_sockets(&pgpool).await?;
    let mut map: HashMap<SocketAddrV4, String> = HashMap::new();
    sockets.iter().for_each(|socket| {
        let sockv4 = SocketAddrV4::from_str(&socket.socket).unwrap();
        map.insert(sockv4, socket.socket_name.clone());
    });
    let latest_socket = sockets.iter().find_map(|sock| {
        if sock.latest {
            return Some(SocketAddrV4::from_str(&sock.socket).unwrap());
        }
        None
    });
    return Ok((map, latest_socket));
}

pub async fn remove_inactive_connection(
    pgpool: actix_web::web::Data<AppState>,
) -> Result<(HashMap<SocketAddrV4, String>, Option<SocketAddrV4>), sqlx::Error> {
    let (mut sockets, mut latest_socket) = load_sockets_from_db(pgpool.clone()).await?;
    let mut inactive_sockets: Vec<SocketAddrV4> = Vec::new();
    for socket in sockets.keys() {
        if !try_connection(*socket).await {
            inactive_sockets.push(*socket);
        }
    }
    for socket in inactive_sockets {
        println!(
            "Inactive connection found, removing socket: {}...",
            socket.to_string()
        );
        sockets.remove(&socket).unwrap();
    }

    if latest_socket.is_some() {
        if !try_connection(latest_socket.unwrap()).await {
            println!(
                "Inactive connection found, removing latest_socket: {}...",
                latest_socket.unwrap().to_string()
            );
            latest_socket = None;
        }
    }
    Ok((sockets, latest_socket))
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
pub fn socket_vec_builder(
    sockets: HashMap<SocketAddrV4, String>,
    latest_socket: Option<SocketAddrV4>,
) -> Vec<Socket> {
    let mut socks: Vec<Socket> = Vec::new();
    for sock in sockets {
        if let Some(latest) = latest_socket {
            if latest == sock.0 {
                socks.push(Socket {
                    id: None,
                    socket_name: sock.1,
                    socket: latest.to_string(),
                    latest: true,
                });
                continue;
            }
        }
        socks.push(Socket {
            id: None,
            socket_name: sock.1,
            socket: sock.0.to_string(),
            latest: false,
        });
    }
    socks
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
                if let Ok(res) = retrieve_socket_from_db(&pgpool, SocketAddrV4::from_str(&to_test.socket).unwrap()).await{
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
