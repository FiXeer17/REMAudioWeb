use super::{
    messages::{Connect,StartStream},
    session::{Disconnect, WsSession},
};
use crate::utils::configs::DEFAULT_SOCKET;
use actix::{
    Actor, Addr, AsyncContext, Context, Handler,
};

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddrV4,
    str::FromStr,
};




pub struct TcpStreamsManager {
    pub streams: HashMap<SocketAddrV4,HashSet<Addr<WsSession>>>,
}
impl TcpStreamsManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::with_capacity(1),
        }
    }
}



impl Actor for TcpStreamsManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, message: Connect, ctx: &mut Self::Context) -> Self::Result {
        if let Some(open_stream) = self.streams.get_mut(
            &message
                .socket
                .unwrap_or(SocketAddrV4::from_str(DEFAULT_SOCKET).unwrap()),
        ) {
            open_stream.insert(message.addr);
        } else {
            let message = StartStream {
                socket: message.socket,
                client: message.addr,
            };
            ctx.address().do_send(message);
        }
    }
}

impl Handler<StartStream> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, message: StartStream, _: &mut Self::Context) -> Self::Result {
        let stream = self
            .streams
            .entry(
                message
                    .socket
                    .unwrap_or(SocketAddrV4::from_str(DEFAULT_SOCKET).unwrap()),
            )
            .or_insert(HashSet::new());
        stream.insert(message.client);
    }
}

impl Handler<Disconnect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.streams.retain(|_, session| {
            session.remove(&msg.addr);
            !session.is_empty()
        });
    }
}
