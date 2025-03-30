use std::{collections::HashSet, net::SocketAddrV4};

use actix::{Actor, Addr, AsyncContext, Handler};
use uuid::Uuid;
use crate::{services::private::app::tcp_handler::tcp_handler::TcpStreamActor, utils::configs::Env};

use super::{super::messages::*, tcp_manager::TcpStreamsManager};

impl Handler<Connect> for TcpStreamsManager {
    type Result = ();
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let default_socket = Env::get_default_socket();
        let socket = &msg
            .socket
            .unwrap_or(default_socket);
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
impl Handler<SetCommand> for TcpStreamsManager{
    type Result = ();
    fn handle(&mut self, msg: SetCommand, _: &mut Self::Context) -> Self::Result {
        let address = &msg.addr;
        let streams = self.streams.clone();
        for stream in streams{
            if stream.1.contains(&address){
                let socket = stream.0;
                let tcp_act: &Addr<TcpStreamActor> = self.streams_actors.get(&socket).unwrap();
                tcp_act.do_send(msg.clone());
            }
        }
    }
}

impl Handler<GetConnections> for TcpStreamsManager{
    type Result = Option<Vec<SocketAddrV4>>;
    fn handle(&mut self, _: GetConnections, _: &mut Self::Context) -> Self::Result {
        let socket_vec : Vec<SocketAddrV4> = self.streams_actors.keys().cloned().collect();
        if socket_vec.is_empty(){
            return None
        }
        Some(socket_vec)
    }
}

impl Handler<SessionOpened> for TcpStreamsManager{
    type Result = String;
    fn handle(&mut self, _: SessionOpened, _: &mut Self::Context) -> Self::Result {
       let mut uuid = Uuid::new_v4();
       while self.uuids.get(&uuid).is_some(){
         uuid = Uuid::new_v4();
       }
       self.uuids.insert(uuid);
       uuid.to_string()
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

impl Handler<CheckSessionUUID> for TcpStreamsManager{
    type Result = bool;
    fn handle(&mut self, msg: CheckSessionUUID, _ctx: &mut Self::Context) -> Self::Result {
        match self.uuids.get(&msg.uuid){
            Some(_) => return true,
            None => return false 
        }
    }
}