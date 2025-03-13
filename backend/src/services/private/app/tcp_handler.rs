use actix::{Actor, Context};



pub struct TcpStreamActor{
    
}

impl Actor for TcpStreamActor{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        
    }
}