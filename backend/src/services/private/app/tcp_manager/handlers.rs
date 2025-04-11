use std::{collections::{HashMap, HashSet}, net::SocketAddrV4, str::FromStr};

use crate::{
    services::private::app::tcp_handler::tcp_handler::TcpStreamActor, utils::common::check_socket,
};
use actix::{Actor, AsyncContext, Handler};
use uuid::Uuid;

use super::{super::messages::*, tcp_manager::TcpStreamsManager};

impl Handler<Connect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let socket = match &msg.socket {
            Some(socket) => *socket,
            None => match self.latest_socket {
                Some(socket) => socket,
                None => {
                    msg.addr.do_send(GeneralConnectionError {
                        socket: None,
                        error: "Cannot find available socket.".to_string(),
                    });
                    return;
                }
            },
        };
        println!("Connecting to {:?}", socket);
        if let Some(open_stream) = self.streams.get_mut(&socket) {
            open_stream.insert(msg.addr.clone());
            let mut message = msg.clone();
            if msg.socket.is_none() {
                message = Connect {
                    addr: msg.addr,
                    socket: Some(socket),
                };
            }
            self.streams_actors.get(&socket).unwrap().do_send(message);
        } else {
            let message = StartStream {
                socket: Some(socket),
                client: msg.addr,
            };
            ctx.address().do_send(message);
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
                self.latest_socket = sockv4;
                if sockv4.is_some(){
                    self.sockets.insert(sockv4.unwrap(), msg.socket_name);
                }
                return true;

            }
        }
        false
    }
}

impl Handler<RemoveSocket> for TcpStreamsManager{
    type Result = ();
    fn handle(&mut self, msg: RemoveSocket, _ctx: &mut Self::Context) -> Self::Result {
        let sessions= self.streams.remove(&msg.socket);
        let message = ClosedByAdmin{};

        if let Some(socket) = self.latest_socket{
            if &socket == &msg.socket{
                self.latest_socket = None;
            }
        }
        if sessions.is_some(){
            sessions.unwrap().iter().for_each(|addr|{
                addr.do_send(message.clone());
            });
        }
        if let Some(stream_actor) = self.streams_actors.remove(&msg.socket){
            stream_actor.do_send(message.clone());
        }
        self.avail_map.remove(&msg.socket);
        let to_remove:Vec<Uuid> = self.uuids_sockets.iter().filter_map(|(uuid,socket)|{
            if let Some(socket) = socket{
                if &check_socket(socket.clone()).unwrap().unwrap() == &msg.socket{
                    return Some(*uuid);
                }
                return None;
            }
            None
        }).collect();
        to_remove.iter().for_each(|uuid|{
            let to_reset = self.uuids_sockets.get_mut(uuid).unwrap();
            *to_reset = None;
        });
        self.sockets.remove(&msg.socket);
    }
}

impl Handler<RetrieveSocket> for TcpStreamsManager {
    type Result = Option<String>;
    fn handle(&mut self, msg: RetrieveSocket, _: &mut Self::Context) -> Self::Result {
        return self.uuids_sockets.get(&msg.uuid).cloned().unwrap();
    }
}

impl Handler<GetConnections> for TcpStreamsManager {
    type Result = Option<HashMap<SocketAddrV4,String>>;
    fn handle(&mut self, _: GetConnections, _: &mut Self::Context) -> Self::Result {
        if self.sockets.is_empty() {
            return None;
        }
        Some(self.sockets.clone())
    }
}
impl Handler<GetLatestConnection> for TcpStreamsManager{
    type Result = Option<HashMap<SocketAddrV4,String>>;
    fn handle(&mut self, _msg: GetLatestConnection, _ctx: &mut Self::Context) -> Self::Result {
        if self.latest_socket.is_some(){
            let sock = self.latest_socket.unwrap();
            let mut latest_socket: HashMap<SocketAddrV4, String> = HashMap::new();
            latest_socket.insert(sock, self.sockets.get(&sock).cloned().unwrap());
            return Some(latest_socket);
        }
        None
    }
}



impl Handler<MatrixPostMiddleware> for TcpStreamsManager{
    type Result = ();
    fn handle(&mut self, msg: MatrixPostMiddleware, _ctx: &mut Self::Context) -> Self::Result {
        let addr = msg.clone().addr.unwrap();
        let actor =self.streams.keys().find_map(|socket|{
            if self.streams.get(socket).unwrap().contains(&addr){
                return self.streams_actors.get(socket);
            }
            None
        });
        if let Some(actor) = actor{
            actor.do_send(msg);
        }
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
        let stream_actor_addr = TcpStreamActor::new(socket, ctx.address()).start();
        self.streams_actors
            .entry(socket)
            .or_insert(stream_actor_addr);
    }
}
impl Handler<PendingConnections> for TcpStreamsManager {
    type Result = bool;
    fn handle(&mut self, _msg: PendingConnections, _ctx: &mut Self::Context) -> Self::Result {
        for socket in self.uuids_sockets.values() {
            if socket.is_some() {
                return true;
            }
        }
        return false;
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
    fn handle(&mut self, msg: StreamFailed, _: &mut Self::Context) -> Self::Result {
        for session in self.streams.remove(&msg.socket).unwrap() {
            session.do_send(msg.clone())
        }
    }
}

impl Handler<ClosedByRemotePeer> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, _: &mut Self::Context) -> Self::Result {
        self.streams_actors.remove(&msg.socket);
        for session in self.streams.remove(&msg.socket).unwrap() {
            session.do_send(msg.clone());
        }
    }
}

// POST-MIDDLEWARE (everything to push in response after the matrix response is recieved)
impl Handler<MatrixReady> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: MatrixReady, _: &mut Self::Context) -> Self::Result {
        for session in self.streams.get(&msg.socket).unwrap().clone() {
            let message = self.post_middleware(msg.clone(), session.clone());
            session.do_send(message);
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
