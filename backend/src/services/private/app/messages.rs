use crate::engine::lib::MatrixCommand;
use crate::AppState;

use super::schemas::MatrixStates;
use super::ws_session::session::WsSession;
use super::ws_session::utils::UpdateVisibility;
use actix::prelude::*;
use actix::Message;
use serde::Serialize;
use tokio::net::TcpStream;
use uuid::Uuid;
use std::{net::SocketAddrV4,collections::HashMap};

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
pub struct SessionOpened{
    pub socket : Option<String>,
    pub user_id: i32,
}

#[derive(Message,Clone)]
#[rtype(result="Option<String>")]
pub struct RetrieveSocket{
    pub uuid: Uuid
}

#[derive(Message,Clone)]
#[rtype(result="bool")]
pub struct CheckSessionUUID{
    pub uuid: Uuid
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct SetHandlerState {
    pub socket: SocketAddrV4,
    pub state: Option<Addr<WsSession>>,
}


#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct HandlerState {
    pub available: bool
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
#[derive(Message,Clone,Serialize)]
#[rtype(result="()")]
pub struct GeneralConnectionError{
    pub socket: Option<SocketAddrV4>,
    pub error: String, 
}
#[derive(Message,Clone,Serialize)]
#[rtype(result="()")]
pub struct GeneralError{
    pub error: String, 
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct CommandError{
    pub command: MatrixCommand
}
#[derive(Message,Clone)]
#[rtype(result="bool")]
pub struct SetSocket{
    pub socket_name: String,
    pub socket: String,
    pub uuid: String
}
#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct RemoveSocket{
    pub socket:SocketAddrV4,
    pub uuid: String
}
#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct SetMessage{
    pub addr: Addr<WsSession>,
    pub command: Commands
}

#[derive(Message,Clone,Debug)]
#[rtype(result="()")]
pub struct ClosedByAdmin{}

#[derive(Clone)]
pub enum Commands{
    SetCommand(SetCommand),
    SetVisibility(UpdateVisibility),
    ReCache
}

#[derive(Debug,Clone)]
pub struct SetCommand{
    pub command: MatrixCommand,
}
#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct SetCommandOk{
    pub cmd: MatrixCommand,
}

#[derive(Message,Clone)]
#[rtype(result="Option<HashMap<SocketAddrV4,String>>")]
pub struct GetConnections{}

#[derive(Message,Clone)]
#[rtype(result="Option<HashMap<SocketAddrV4,String>>")]
pub struct GetLatestConnection{}

#[derive(Message,Clone)]
#[rtype(result="bool")]
pub struct PendingConnections{}

#[derive(Message,Clone)]
#[rtype(result="Option<i32>")]
pub struct RetrieveUserFromUuid{
    pub uuid:Uuid,
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct MatrixPostMiddleware{
    pub addr: Option<Addr<WsSession>>,
    pub states: MatrixStates,
    pub pgpool : actix_web::web::Data<AppState>,
}


