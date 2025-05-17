use actix::{Actor, AsyncContext, Handler};
use futures_util::future::LocalBoxFuture;
use tokio::sync::broadcast;

use crate::services::private::stream::{
    messages::*, stream_handler::stream_handler::StreamHandler,
};

use super::streams_manager::StreamManager;

impl Handler<Connect> for StreamManager {
    type Result = LocalBoxFuture<'static, Result<broadcast::Sender<Vec<u8>>, ()>>;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let socket_clone = msg.socket.clone();
        let ctx_addr = ctx.address();

        let stream_actor = if let Some(existing) = self.open_streams.get(&msg.socket) {
            existing.clone()
        } else {
            let new_actor = StreamHandler::new(socket_clone.clone(), ctx_addr).start();
            self.open_streams
                .insert(socket_clone.clone(), new_actor.clone());
            new_actor
        };

        Box::pin(async move {
            match stream_actor.send(Connect {socket: socket_clone,}).await{
                Ok(tx) => tx,
                Err(_) => Err(()),
            }
        })
    }
}
