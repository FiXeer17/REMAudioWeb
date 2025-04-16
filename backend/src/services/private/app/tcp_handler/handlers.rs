use std::sync::Arc;

use actix::{ActorContext, AsyncContext, Handler};
use futures_util::lock::Mutex;

use crate::utils::configs::tcp_comunication_settings;

use super::super::messages::*;
use super::tcp_handler::TcpStreamActor;
use super::utils::command_polling;

impl Handler<StreamStarted> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: StreamStarted, ctx: &mut Self::Context) -> Self::Result {
        let socket = self.stream_socket.clone();
        let stream = Arc::new(Mutex::new(msg.tcp_stream));
        let ctx_addr = ctx.address().clone();
        self.stream = Some(stream.clone());
        tokio::spawn(async move {
            TcpStreamActor::read_states(ctx_addr, socket.clone(), stream).await;
        });
    }
}

impl Handler<StreamFailed> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: StreamFailed, ctx: &mut Self::Context) -> Self::Result {
        self.tcp_manager.do_send(msg);
        ctx.stop();
    }
}
impl Handler<ClosedByRemotePeer> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, ctx: &mut Self::Context) -> Self::Result {
        self.tcp_manager.do_send(msg);
        ctx.stop();
    }
}

impl Handler<ClosedByAdmin> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, _msg: ClosedByAdmin, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}
impl Handler<MatrixPostMiddleware> for TcpStreamActor{
    type Result = ();
    fn handle(&mut self, msg: MatrixPostMiddleware, _ctx: &mut Self::Context) -> Self::Result {
        self.machine_states = Some(msg.states);
    }
}

impl Handler<MatrixReady> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: MatrixReady, ctx: &mut Self::Context) -> Self::Result {
        self.machine_states = Some(msg.states.clone());
        self.tcp_manager.do_send(msg);
        if self.cmd_poller.is_none() {
            let cmd_poller = ctx.run_interval(tcp_comunication_settings::get_command_delay(), command_polling);
            self.cmd_poller = Some(cmd_poller);
        }
    }
}

impl Handler<Connect> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        if self.machine_states.is_none() {
            return;
        }
        let states = self.machine_states.clone().unwrap();
        let message = MatrixReady {
            socket: msg.socket.unwrap(),
            states,
        };
        self.tcp_manager.do_send(message);
    }
}

impl Handler<SetMessage> for TcpStreamActor{
    type Result = ();
    fn handle(&mut self, msg: SetMessage, ctx: &mut Self::Context) -> Self::Result {
        self.watch_inactive(ctx, msg.addr.clone());
        match msg.command{
            Commands::SetCommand(sc) => self.handle_set_command(sc),
            Commands::SetVisibility(sv) => {if self.machine_states.is_some(){
                let machine_sates = self.machine_states.as_ref().unwrap();
                if machine_sates.i_visibility.is_none() || machine_sates.o_visibility.is_none(){
                    let message = MatrixReady{socket:self.stream_socket,states:machine_sates.clone()};
                    self.tcp_manager.do_send(message);
                }else{
                    self.handle_set_visibility_command(sv.set_visibility,sv.db,msg.addr,ctx.address());
                }
            }},
            Commands::ReCache => self.handle_recache(ctx),
        }
    }
}