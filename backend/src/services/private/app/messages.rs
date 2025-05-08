use crate::engines::audio_engine::lib::MatrixCommand;
use crate::services::private::socket::utils::Device;
use crate::services::public::schemas::Socket;

use super::schemas::CameraStates;
use super::schemas::MatrixStates;
use super::schemas::SetAttributes;
use super::ws_session::session::WsSession;
use actix::prelude::*;
use actix::Message;
use serde::Serialize;
use tokio::net::TcpStream;
use uuid::Uuid;
use std::collections::VecDeque;
use std::{net::SocketAddrV4,collections::HashSet};

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartStream {
    pub socket: Option<SocketAddrV4>,
    pub client: Addr<WsSession>,
    pub device_type: Device,
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
pub struct CameraReady{
    pub socket: SocketAddrV4,
    pub states: CameraStates
}
#[derive(Message,Clone)]
#[rtype(result="()")]
pub enum DeviceReady{
    MatrixReady(MatrixReady),
    CameraReady(CameraReady)
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
    pub socket: Option<SocketAddrV4>,
    pub error: String, 
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct CommandError{
    pub command: MatrixCommand
}
#[derive(Message,Clone,Debug)]
#[rtype(result="bool")]
pub struct SetSocket{
    pub socket_name: String,
    pub socket: String,
    pub device: String,
    pub uuid: String
}
#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct RemoveSocket{
    pub socket:SocketAddrV4,
}
#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct SetMessage{
    pub addr: Addr<WsSession>,
    pub command: Commands
}

#[derive(Message,Clone,Debug)]
#[rtype(result="()")]
pub struct ClosedByAdmin{
    pub sessions: Option<HashSet<Addr<WsSession>>>,
    pub device: Option<Device>,
}

#[derive(Clone)]
pub enum Commands{
    SetMatrixCommand(SetCommand),
    SetVisibility(SetAttributes),
    SetChannelLabel(SetAttributes),
    SetPresetLabel(SetAttributes),
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
#[rtype(result="Option<HashSet<Socket>>")]
pub struct GetConnections{}

#[derive(Message,Clone)]
#[rtype(result="Option<i32>")]
pub struct RetrieveUserFromUuid{
    pub uuid:Uuid,
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct UnavailableSockets{
    pub sockets: Vec<Socket>,
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct SocketRestarted{
    pub socket: Option<Socket>,
    pub latest_socket:Option<Socket>
}

#[derive(Message,Clone)]
#[rtype(result="()")]
pub struct InactiveQueue{
    pub queue: VecDeque<Socket>
}


