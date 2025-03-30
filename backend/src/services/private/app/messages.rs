use crate::engine::lib::MatrixCommand;

use super::schemas::MatrixStates;
use super::ws_session::session::WsSession;
use actix::prelude::*;
use actix::Message;
use tokio::net::TcpStream;
use uuid::Uuid;
use std::net::SocketAddrV4;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub addr: Addr<WsSession>,
}

#[derive(Message, Debug,Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WsSession>,
    pub socket: Option<SocketAddrV4>,
}

#[derive(Message,Clone)]
#[rtype(result="String")]
pub struct SessionOpened{}

#[derive(Message,Clone)]
#[rtype(result="bool")]
pub struct CheckSessionUUID{
    pub uuid: Uuid
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


#[derive(Message)]
#[rtype(result="()")]
pub struct StreamStarted{
    pub tcp_stream : TcpStream
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct StreamFailed{
    pub socket : SocketAddrV4,
    pub error: String
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct CommandReturn{
    pub response: String,
}


#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct ClosedByRemotePeer{
    pub socket: SocketAddrV4,
    pub message: String
}


#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct MatrixReady{
    pub socket: SocketAddrV4,
    pub states: MatrixStates
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct CommandError{
    pub command: MatrixCommand
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct SetCommand{
    pub command: MatrixCommand,
    pub addr: Addr<WsSession>
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct SetCommandOk{
    pub cmd: MatrixCommand,
}

#[derive(Message,Clone)]
#[rtype(result="Option<Vec<SocketAddrV4>>")]
pub struct GetConnections{}

