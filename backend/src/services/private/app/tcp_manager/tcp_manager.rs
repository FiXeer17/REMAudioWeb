use crate::services::private::app::ws_session::session::WsSession;

use super::super::tcp_handler::tcp_handler::TcpStreamActor;
use actix::{Actor, Addr, Context};
use uuid::Uuid;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddrV4,
};

pub struct TcpStreamsManager {
    pub streams: HashMap<SocketAddrV4, HashSet<Addr<WsSession>>>,
    pub streams_actors: HashMap<SocketAddrV4, Addr<TcpStreamActor>>,
    pub uuids: HashSet<Uuid>,
}

impl TcpStreamsManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::with_capacity(1),
            streams_actors: HashMap::with_capacity(1),
            uuids: HashSet::new(),
        }
    }
}

impl Actor for TcpStreamsManager {
    type Context = Context<Self>;
}
