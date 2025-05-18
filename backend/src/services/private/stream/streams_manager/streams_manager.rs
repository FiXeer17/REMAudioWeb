use std::{collections::HashMap, net::SocketAddrV4};

use actix::{Actor, Addr, Context};

use crate::services::private::stream::stream_handler::stream_handler::StreamHandler;

pub struct StreamManager{
    pub open_streams: HashMap<SocketAddrV4,Addr<StreamHandler>> 
}

impl StreamManager{
    pub fn new() -> Self {
        Self { open_streams: HashMap::new() }
    }
}

impl Actor for StreamManager{
    type Context = Context<Self>;
}

