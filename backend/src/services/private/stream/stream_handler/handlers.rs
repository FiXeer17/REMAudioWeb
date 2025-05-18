use actix::{AsyncContext, Handler, MessageResult, WrapFuture};


use crate::services::private::stream::messages::*;

use super::{stream_handler::StreamHandler, utils::bufferer};



impl Handler<Connect> for StreamHandler{
    type Result = MessageResult<Connect>;
    fn handle(&mut self, _msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(Ok(self.tx.clone()))
    }
}

impl Handler<ReadStdout> for StreamHandler{
    type Result = ();
    fn handle(&mut self, _msg: ReadStdout, ctx: &mut Self::Context) -> Self::Result {
        let process = self.ffmpeg_process.as_mut().unwrap();
        let mut stdout = process.stdout.take().unwrap();
        let tx = self.tx.clone();
        let mut buffer = Vec::new();

        let future = async move {bufferer(&mut buffer, &mut stdout, tx).await};
        self.bufferer = Some(ctx.spawn(future.into_actor(self)));
    }
}