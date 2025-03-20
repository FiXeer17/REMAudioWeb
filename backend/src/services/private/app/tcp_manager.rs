use super::{
    messages::{ClosedByRemotePeer, Connect, StartStream, StreamFailed,MatrixReady},
    session::{Disconnect, WsSession},
    tcp_handler::TcpStreamActor,
};
use crate::utils::configs::Env;
use actix::{Actor, Addr, AsyncContext, Context, Handler};

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddrV4,
    str::FromStr,
};


pub struct TcpStreamsManager {
    pub streams: HashMap<SocketAddrV4, HashSet<Addr<WsSession>>>,
    pub streams_actors: HashMap<SocketAddrV4, Addr<TcpStreamActor>>,
}
impl TcpStreamsManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::with_capacity(1),
            streams_actors: HashMap::with_capacity(1),
        }
    }
}

impl Actor for TcpStreamsManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let default_socket = Env::get_default_socket();
        let socket = &msg
            .socket
            .unwrap_or(SocketAddrV4::from_str(&default_socket).unwrap());
        if let Some(open_stream) = self.streams.get_mut(socket) {
            open_stream.insert(msg.addr.clone());
            let mut message = msg.clone();
            if msg.socket.is_none(){
                message = Connect{addr:msg.addr,socket:Some(*socket)};
            }
            self.streams_actors.get(socket).unwrap().do_send(message);
            
        } else {
            let message = StartStream {
                socket: Some(*socket),
                client: msg.addr,
            };
            ctx.address().do_send(message);
        }
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

impl Handler<Disconnect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let streams = self.streams.clone();
        for mut session in streams{
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

impl Handler<ClosedByRemotePeer> for TcpStreamsManager{
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, _: &mut Self::Context) -> Self::Result {
        for session in self.streams.remove(&msg.socket).unwrap(){
            session.do_send(msg.clone());
        }
    }
}

impl Handler<MatrixReady> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: MatrixReady, _: &mut Self::Context) -> Self::Result {
        for session in self.streams.get(&msg.socket).unwrap(){
            session.do_send(msg.clone());
        }
    }
    
}