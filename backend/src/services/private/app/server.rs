use super::session::WsSession;
use actix::prelude::*;
use actix::Message;
use std::collections::HashSet;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WsSession>,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub message: String,
}

pub struct WsServer {
    pub clients: HashSet<Addr<WsSession>>,
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl WsServer {
    pub fn new() -> Self {
        WsServer {
            clients: HashSet::new(),
        }
    }
}

impl Handler<Connect> for WsServer {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let address = msg.addr;
        self.clients.insert(address);

        /*  TODO: let datas = read_all();
        for client in &self.clients{

            let message = BroadcastMessage{
                message:datas.clone()
            };
            client.do_send(message);
        } */

        for client in &self.clients {
            client.do_send(BroadcastMessage {
                message: "new".to_string(),
            });
        }
    }
}
