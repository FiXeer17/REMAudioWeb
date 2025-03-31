use actix::{ActorContext, Handler};

use super::tcp_handler::TcpStreamActor;
use super::super::messages::*;

impl Handler<SetCommand> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: SetCommand, _: &mut Self::Context) -> Self::Result {
        self.commands_queue.push_front(msg.command);
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

impl Handler<MatrixReady> for TcpStreamActor {
    type Result = ();
    fn handle(&mut self, msg: MatrixReady, _: &mut Self::Context) -> Self::Result {
        self.machine_states = Some(msg.states.clone());
        self.tcp_manager.do_send(msg);
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