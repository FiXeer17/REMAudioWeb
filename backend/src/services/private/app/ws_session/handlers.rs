use std::time::Instant;

use crate::services::public::interfaces::retrieve_socketid_from_db;
use crate::utils::common::return_json_reason;
use actix::{ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use serde_json::json;

use super::super::messages::*;
use super::session::WsSession;
use super::utils::{check_channel, HandleText, UpdateVisibility};

impl Handler<StreamFailed> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: StreamFailed, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(return_json_reason(&msg.error.to_string()).to_string());
        ctx.stop();
    }
}
impl Handler<ClosedByRemotePeer> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ClosedByRemotePeer, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(return_json_reason(&msg.message.to_string()).to_string());

        ctx.stop();
    }
}

impl Handler<ClosedByAdmin> for WsSession{
    type Result = ();
    fn handle(&mut self, _msg: ClosedByAdmin, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(json!({"reason":"socket deleted by admin."}).to_string());
        ctx.stop();
    }
}


// POST-MIDDLEWARE 
impl Handler<MatrixReady> for WsSession {
    type Result = ();
    fn handle(&mut self, msg:MatrixReady, ctx: &mut Self::Context) -> Self::Result {
        let addr = ctx.address().clone();
        let pgpool = self.pgpool.clone();
        if msg.states.i_visibility.is_none() || msg.states.o_visibility.is_none(){
            tokio::spawn(async move {
                let socket_id = retrieve_socketid_from_db(&pgpool, msg.socket).await;
                if socket_id.is_err(){
                    println!("cannot retrieve socket id from database");
                    return;
                }
                let states = WsSession::attach_channel_visibility(&mut msg.states.clone(),socket_id.unwrap(),pgpool.clone()).await;
                if let Err(_) = states{
                    addr.do_send(GeneralError{error:"cannot attach visibility.".to_string()});
                    return;
                }
                addr.do_send(MatrixPostMiddleware{addr:None,states:states.unwrap(),pgpool});
            });
        }else{
            let message = serde_json::to_string_pretty(&msg.states).unwrap();
            ctx.text(message);
        }
    }
}
impl Handler<MatrixPostMiddleware> for WsSession{
    type Result = ();
    fn handle(&mut self, msg: MatrixPostMiddleware, ctx: &mut Self::Context) -> Self::Result {
        self.srv.do_send(MatrixPostMiddleware{addr:Some(ctx.address()),states:msg.states.clone(),pgpool:msg.pgpool});
        let message = serde_json::to_string_pretty(&msg.states).unwrap();
        ctx.text(message);
    }
}
impl Handler<GeneralError> for WsSession{
    type Result = ();
    fn handle(&mut self, msg: GeneralError, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(return_json_reason(&msg.error).to_string());
    }
}

impl Handler<GeneralConnectionError> for WsSession{
    type Result = ();
    fn handle(&mut self, msg: GeneralConnectionError, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(return_json_reason(&msg.error).to_string());
        ctx.stop();
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                self.hb = Instant::now();
                let addr = ctx.address();
                match self.deserialize_text(text.to_string()) {
                    HandleText::Command(cmd) => match cmd {
                        Ok(cmd) => {
                            let msg = SetCommand {
                                command: cmd,
                            };
                            self.srv.do_send(SetMessage{ addr, command: Commands::SetCommand(msg)});
                        },
                        Err(e) => {
                            ctx.text(e.to_string());
                        }
                    },
                    
                    HandleText::Recache => {
                        self.srv.do_send(SetMessage{addr,command:Commands::ReCache});
                    },
                    HandleText::Error(reason) => {
                        ctx.text(return_json_reason(&reason).to_string());
                    },
                    HandleText::SetVisibility(sv) => {
                        let sv_clone = sv.clone();
                        let channel = sv_clone.channel.parse::<u8>().unwrap();
                        let io = sv_clone.io;
                        if check_channel(io,channel){
                        self.srv.do_send(SetMessage{addr,command:Commands::SetVisibility(UpdateVisibility{db:self.pgpool.clone(),set_visibility:sv,user_id:self.user_id})});
                        }
                    }
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

