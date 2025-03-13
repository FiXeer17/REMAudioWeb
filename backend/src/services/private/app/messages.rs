use super::session::WsSession;
use actix::prelude::*;
use actix::Message;
use std::net::SocketAddrV4;


#[derive(Message, Debug,Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WsSession>,
    pub socket: Option<SocketAddrV4>,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub message: String,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct StartStream {
    pub socket: Option<SocketAddrV4>,
    pub client: Addr<WsSession>,
}




