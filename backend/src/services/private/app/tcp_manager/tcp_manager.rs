use crate::services::private::app::{
    messages::{MatrixReady, SetMessage},
    ws_session::session::WsSession,
};

use super::{super::tcp_handler::tcp_handler::TcpStreamActor, utils::attach_availability};
use actix::{Actor, Addr, Context};
use uuid::Uuid;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddrV4,
};

pub struct TcpStreamsManager {
    pub streams: HashMap<SocketAddrV4, HashSet<Addr<WsSession>>>,
    pub streams_actors: HashMap<SocketAddrV4, Addr<TcpStreamActor>>,
    pub uuids: HashMap<Uuid, Option<String>>,
    pub avail_map: HashMap<SocketAddrV4, Option<Addr<WsSession>>>,
}

impl TcpStreamsManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::with_capacity(1),
            streams_actors: HashMap::with_capacity(1),
            uuids: HashMap::new(),
            avail_map: HashMap::new(),
        }
    }
    pub fn handle_message(&self, msg: SetMessage) {
        let addr = &msg.addr;
        for stream in &self.streams {
            if stream.1.contains(addr) {
                let socket = stream.0;
                let tcp_actor = self.streams_actors.get(socket).unwrap();
                let availability = self.avail_map.get(socket).unwrap();
                if let Some(wsocket) = availability {
                    if wsocket == &msg.addr {
                        tcp_actor.do_send(msg.clone());
                    }
                } else {
                    tcp_actor.do_send(msg.clone());
                }
            }
        }
    }
    pub fn post_middleware(&mut self, msg: MatrixReady, session: Addr<WsSession>) -> MatrixReady {
        self.avail_map.entry(msg.socket).or_insert(None);
        let availability = self.avail_map.get(&msg.socket).unwrap();

        let mut states = msg.states;
        states = attach_availability(states, availability, &session);

        MatrixReady {
            states,
            socket: msg.socket,
        }
    }
}

impl Actor for TcpStreamsManager {
    type Context = Context<Self>;
}
