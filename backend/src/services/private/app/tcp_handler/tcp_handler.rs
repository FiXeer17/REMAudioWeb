use super::super::messages::{StreamFailed, StreamStarted};

use crate::{
    configs::tcp_comunication_settings,
    services::private::{
        app::{
            schemas::{DeviceCommnd, MachineStates},
            tcp_manager::tcp_manager::TcpStreamsManager,
        },
        socket::utils::Device,
    },
    AppState,
};
use actix::{Actor, Addr, AsyncContext, Context, SpawnHandle};
use actix_web::web::Data;
use futures_util::lock::Mutex;
use log::warn;
use std::{collections::VecDeque, net::SocketAddrV4, sync::Arc};
use tokio::net::TcpStream;

pub struct TcpStreamActor {
    pub stream_socket: SocketAddrV4,
    pub tcp_manager: Addr<TcpStreamsManager>,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
    pub commands_queue: VecDeque<DeviceCommnd>,
    pub machine_states: Option<MachineStates>,
    pub cmd_poller: Option<SpawnHandle>,
    pub owner: Option<SpawnHandle>,
    pub pgpool: Data<AppState>,
    pub device_type: Device,
}

impl TcpStreamActor {
    pub fn new(
        stream_socket: SocketAddrV4,
        tcp_manager: Addr<TcpStreamsManager>,
        pgpool: actix_web::web::Data<AppState>,
        device_type: Device,
    ) -> Self {
        Self {
            stream_socket,
            tcp_manager,
            stream: None,
            commands_queue: VecDeque::new(),
            machine_states: None,
            cmd_poller: None,
            owner: None,
            pgpool,
            device_type,
        }
    }
}

impl Actor for TcpStreamActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let socket = self.stream_socket.clone();
        let ctx_address = ctx.address().clone();
        tokio::spawn(async move {
            let mut retries: u8 = 0;
            while retries <= tcp_comunication_settings::get_max_connection_retries() {
                match tokio::time::timeout(
                    tcp_comunication_settings::get_connection_timeout(),
                    TcpStream::connect(socket.to_string()),
                )
                .await
                {
                    Ok(not_timedout) => match not_timedout {
                        Ok(tcp_stream) => {
                            tcp_stream.set_nodelay(false).unwrap();
                            let message = StreamStarted { tcp_stream };
                            ctx_address.do_send(message);
                            break;
                        }
                        Err(e) => {
                            if retries == tcp_comunication_settings::get_max_connection_retries() {
                                let message = StreamFailed {
                                    socket,
                                    error: e.to_string(),
                                };
                                ctx_address.do_send(message);
                                println!("cannot create tcp stream, closing...");
                                return;
                            } else {
                                retries += 1;
                                tokio::time::sleep(tcp_comunication_settings::get_reconnect_delay()).await;
                            }
                        }
                    },
                    Err(t) => {
                        if retries == tcp_comunication_settings::get_max_connection_retries() {
                            let message = StreamFailed {
                                socket,
                                error: t.to_string(),
                            };
                            ctx_address.do_send(message);
                            warn!("Cannot create tcp stream (time elapsed), closing...");
                            return;
                        }
                        retries += 1;
                    }
                };
            }
        });
    }
}
