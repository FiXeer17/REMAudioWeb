use std::{net::SocketAddrV4,};
use tokio::sync::broadcast;
use actix::Message;


#[derive(Message,Debug,Clone)]
#[rtype(result="Result<broadcast::Sender<Vec<u8>>,()>")]
pub struct Connect{
    pub socket : SocketAddrV4
}

#[derive(Message,Debug,Clone)]
#[rtype(result="()")]
pub struct StropStream {}

#[derive(Message,Debug,Clone)]
#[rtype(result="()")]

pub struct ReadStdout{}