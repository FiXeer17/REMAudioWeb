use std::{collections::HashSet, net::SocketAddrV4, str::FromStr};

use crate::{
    services::{
        private::{
            app::{
                tcp_handler::{
                    tcp_handler::TcpStreamActor,
                    utils::{add_channels, add_presets},
                },
                utils::HasStatesMessage,
            },
            socket::utils::Device,
        },
        public::{
            interfaces::{insert_socket_in_db, retrieve_socket_from_db},
            schemas::{IsContainedExt, Socket},
        },
    },
    utils::common::check_socket,
};
use actix::{Actor, AsyncContext, Handler};
use log::{info, warn};
use uuid::Uuid;

use super::{super::messages::*, tcp_manager::TcpStreamsManager};

impl Handler<Connect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let to_connect: Vec<SocketAddrV4> = match &msg.socket {
            Some(socket) => vec![*socket],
            None => {
                let latest_sockets: Vec<SocketAddrV4> =
                    vec![self.latest_audio_socket, self.latest_video_socket]
                        .into_iter()
                        .flatten()
                        .collect();
                if latest_sockets.is_empty() {
                    msg.addr.do_send(GeneralConnectionError {
                        socket: None,
                        error: "Cannot find available socket.".to_string(),
                    });
                    return;
                }
                latest_sockets
            }
        };

        let pool_cloned = self.pgpool.clone();
        let to_connect_clone = to_connect.clone();
        tokio::spawn(async move {
            for socket in to_connect_clone {
                let Ok(dbsock) = retrieve_socket_from_db(&pool_cloned, socket).await else {
                    warn!("Couldn't retrieve socket from database.");
                    return;
                };
                let pool_for_presets = pool_cloned.clone();
                add_presets(pool_for_presets.clone(), socket, dbsock.device).await;
                add_channels(pool_for_presets, socket).await;
            }
        });
        for socket in to_connect {
            info!("Connecting to {:?}", socket);
            let addr_clone = msg.addr.clone();
            if let Some(open_stream) = self.streams.get_mut(&socket) {
                let mut message = msg.clone();
                open_stream.insert(addr_clone.clone());

                if msg.socket.is_none() {
                    message = Connect {
                        addr: addr_clone,
                        socket: Some(socket),
                    };
                }
                self.streams_actors.get(&socket).unwrap().do_send(message);
            } else {
                let ctx_addr = ctx.address().clone();
                let pool_cloned = self.pgpool.clone();

                tokio::spawn(async move {
                    info!(
                        "New connection detected at socket {}, processing...",
                        socket.to_string()
                    );
                    if let Ok(dbsock) = retrieve_socket_from_db(&pool_cloned, socket).await {
                        let Ok(device_type) = Device::from_str(&dbsock.device) else {
                            return;
                        };
                        let message = StartStream {
                            socket: Some(socket),
                            client: addr_clone,
                            device_type,
                        };
                        ctx_addr.do_send(message);
                    }
                });
            }
        }
    }
}
impl Handler<SetSocket> for TcpStreamsManager {
    type Result = bool;
    fn handle(&mut self, msg: SetSocket, _: &mut Self::Context) -> Self::Result {
        if let Ok(uuid) = Uuid::from_str(&msg.uuid) {
            if let Some(socket) = self.uuids_sockets.get_mut(&uuid) {
                *socket = Some(msg.socket.clone());
                let sockv4 = check_socket(msg.socket).unwrap();

                if msg.device == Device::Audio.to_string() {
                    self.latest_audio_socket = sockv4;
                } else if msg.device == Device::Video.to_string() {
                    self.latest_video_socket = sockv4
                }

                if let Some(sockv4) = sockv4 {
                    let new_sockets: HashSet<Socket> = self
                        .sockets
                        .to_owned()
                        .into_iter()
                        .map(|mut sock| {
                            if sock.device == msg.device {
                                sock.latest = false;
                            }
                            sock
                        })
                        .collect();
                    self.sockets = new_sockets;
                    let socket = Socket {
                        id: None,
                        socket_name: msg.socket_name.clone(),
                        socket: sockv4.to_string(),
                        latest: true,
                        device: msg.device.clone(),
                        latest_preset: None,
                    };
                    self.sockets.remove(&socket);
                    self.sockets.insert(socket);

                    let pool = self.pgpool.clone();
                    tokio::spawn(async move {
                        info!("Setting the new socket...");
                        let result =
                            insert_socket_in_db(&pool, msg.socket_name, sockv4.clone(), msg.device)
                                .await;
                        if result.is_err() {
                            warn!("Couldn't save socket in database");
                        }
                        info!("Socket set succesfully.");
                    });
                }
                return true;
            }
        }
        false
    }
}

impl Handler<RemoveSocket> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: RemoveSocket, _ctx: &mut Self::Context) -> Self::Result {
        let sessions = self.streams.remove(&msg.socket);
        if msg.forced {
            let message = ClosedByAdmin {device:None,sessions};
            let stream_actor= self.streams_actors.get(&msg.socket).unwrap();
            stream_actor.do_send(message.clone());  
        }
        let index = self
            .inactive_sockets
            .iter()
            .position(|s| s.socket == msg.socket.to_string());
        if let Some(index) = index {
            self.inactive_sockets.remove(index);
        }

        if let Some(socket) = self.latest_audio_socket {
            if &socket == &msg.socket {
                self.latest_audio_socket = None;
            }
        }

        if let Some(socket) = self.latest_video_socket {
            if &socket == &msg.socket {
                self.latest_video_socket = None;
            }
        }

        self.streams_actors.remove(&msg.socket);

        self.avail_map.remove(&msg.socket);
        let to_remove: Vec<Uuid> = self
            .uuids_sockets
            .iter()
            .filter_map(|(uuid, socket)| {
                if let Some(socket) = socket {
                    if &check_socket(socket.clone()).unwrap().unwrap() == &msg.socket {
                        return Some(*uuid);
                    }
                    return None;
                }
                None
            })
            .collect();
        to_remove.iter().for_each(|uuid| {
            let to_reset = self.uuids_sockets.get_mut(uuid).unwrap();
            *to_reset = None;
        });
        self.sockets
            .retain(|s| SocketAddrV4::from_str(&s.socket).unwrap() != msg.socket);
        if !msg.forced {
            if let Some(socket) = self.sockets.socket_is_contained(&msg.socket.to_string()) {
                self.inactive_sockets.push_back(socket);
            }
        }
    }
}

impl Handler<RetrieveSocket> for TcpStreamsManager {
    type Result = Option<String>;
    fn handle(&mut self, msg: RetrieveSocket, _: &mut Self::Context) -> Self::Result {
        return self.uuids_sockets.get(&msg.uuid).cloned().unwrap();
    }
}

impl Handler<GetConnections> for TcpStreamsManager {
    type Result = Option<HashSet<Socket>>;
    fn handle(&mut self, _: GetConnections, _: &mut Self::Context) -> Self::Result {
        if self.sockets.is_empty() {
            return None;
        }
        Some(self.sockets.clone())
    }
}

impl Handler<InactiveQueue> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: InactiveQueue, _ctx: &mut Self::Context) -> Self::Result {
        self.inactive_sockets = msg.queue;
    }
}

impl Handler<SessionOpened> for TcpStreamsManager {
    type Result = String;
    fn handle(&mut self, msg: SessionOpened, _: &mut Self::Context) -> Self::Result {
        let mut uuid = Uuid::new_v4();
        while self.uuids_sockets.get(&uuid).is_some() {
            uuid = Uuid::new_v4();
        }
        self.uuids_sockets.insert(uuid, None);
        self.uuids_users.insert(uuid, msg.user_id);
        uuid.to_string()
    }
}

impl Handler<GeneralError> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: GeneralError, ctx: &mut Self::Context) -> Self::Result {
        if let Some(sock) = msg.socket {
            self.streams.get(&sock).unwrap().iter().for_each(|addr| {
                addr.do_send(msg.clone());
            });
            ctx.address().do_send(RemoveSocket {
                socket: sock,
                forced: false,
            });
        }
    }
}

impl Handler<RetrieveUserFromUuid> for TcpStreamsManager {
    type Result = Option<i32>;
    fn handle(&mut self, msg: RetrieveUserFromUuid, _ctx: &mut Self::Context) -> Self::Result {
        self.uuids_users.get(&msg.uuid).cloned()
    }
}
impl Handler<StartStream> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, message: StartStream, ctx: &mut Self::Context) -> Self::Result {
        let socket = message.socket.unwrap();
        let stream = self.streams.entry(socket).or_insert(HashSet::new());
        stream.insert(message.client);
        let stream_actor_addr = TcpStreamActor::new(
            socket,
            ctx.address(),
            self.pgpool.clone(),
            message.device_type,
        )
        .start();
        self.streams_actors
            .entry(socket)
            .or_insert(stream_actor_addr);
        info!("New TCP handler started at: {}", socket.to_string());
    }
}

impl Handler<SetHandlerState> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: SetHandlerState, _: &mut Self::Context) -> Self::Result {
        self.avail_map.insert(msg.socket.clone(), msg.state.clone());
    }
}

impl Handler<Disconnect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let streams = self.streams.clone();
        for mut session in streams {
            session.1.remove(&msg.addr.clone());
        }
    }
}

impl Handler<StreamFailed> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: StreamFailed, ctx: &mut Self::Context) -> Self::Result {
        for session in self.streams.get(&msg.socket).unwrap() {
            session.do_send(msg.clone())
        }
        ctx.address().do_send(RemoveSocket {
            socket: msg.socket,
            forced: false,
        });
    }
}

impl Handler<ClosedByRemotePeer> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, ctx: &mut Self::Context) -> Self::Result {
        if let Some(removed) = self.streams.get(&msg.socket) {
            for session in removed {
                session.do_send(msg.clone());
            }
        }
        ctx.address().do_send(RemoveSocket {
            socket: msg.socket,
            forced: false,
        });
    }
}

// POST-MIDDLEWARE (everything to push in response after the matrix response is recieved)
impl Handler<DeviceReady> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: DeviceReady, _: &mut Self::Context) -> Self::Result {
        let socket = msg.get_socket();
        if let Some(sessions) = self.streams.get(&socket).cloned() {
            for session in sessions {
                let message = self.post_middleware(msg.clone(), session.clone());
                session.do_send(message);
            }
        }
    }
}

impl Handler<CheckSessionUUID> for TcpStreamsManager {
    type Result = bool;
    fn handle(&mut self, msg: CheckSessionUUID, _ctx: &mut Self::Context) -> Self::Result {
        self.uuids_sockets.contains_key(&msg.uuid)
    }
}

impl Handler<SetMessage> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: SetMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.handle_message(msg);
    }
}

impl Handler<UnavailableSockets> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: UnavailableSockets, ctx: &mut Self::Context) -> Self::Result {
        msg.sockets.into_iter().for_each(|s| {
            let to_remove = SocketAddrV4::from_str(&s.socket).unwrap();
            if let Some(soc) = self.latest_audio_socket {
                if soc.to_string() == s.socket {
                    self.latest_audio_socket = None;
                }
            }
            if let Some(soc) = self.latest_video_socket {
                if soc.to_string() == s.socket {
                    self.latest_video_socket = None;
                }
            }
            ctx.address().do_send(ClosedByRemotePeer {
                socket: to_remove,
                message: "unknown".to_string(),
            });

            self.avail_map.remove(&to_remove);
            self.sockets.retain(|s| s.socket != to_remove.to_string());

            let to_remove_uuids: Vec<Uuid> = self
                .uuids_sockets
                .iter()
                .filter_map(|(uuid, socket)| {
                    if let Some(socket) = socket {
                        if &check_socket(socket.clone()).unwrap().unwrap() == &to_remove {
                            return Some(*uuid);
                        }
                        return None;
                    }
                    None
                })
                .collect();
            to_remove_uuids.iter().for_each(|uuid| {
                let to_reset = self.uuids_sockets.get_mut(uuid).unwrap();
                *to_reset = None;
            });
            self.inactive_sockets.push_front(s);
        });
    }
}

impl Handler<SocketRestarted> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: SocketRestarted, _ctx: &mut Self::Context) -> Self::Result {
        let avail_socket: Socket;
        if let Some(socket) = msg.latest_socket {
            if socket.device == Device::Audio.to_string() {
                self.latest_audio_socket = Some(SocketAddrV4::from_str(&socket.socket).unwrap());
            } else {
                self.latest_video_socket = Some(SocketAddrV4::from_str(&socket.socket).unwrap());
            }
            avail_socket = socket;
        } else {
            avail_socket = msg.socket.unwrap();
        }
        self.sockets.insert(avail_socket);
    }
}
