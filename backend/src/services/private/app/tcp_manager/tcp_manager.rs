use crate::{
    services::{
        private::app::{
            messages::{MatrixReady, SetMessage, UnavailableSockets},
            ws_session::session::WsSession,
        },
        public::{interfaces::retrieve_sockets, schemas::Socket},
    },
    configs::ping_socket_settings,
    AppState,
};

use super::{
    super::tcp_handler::tcp_handler::TcpStreamActor,
    utils::{attach_availability, remove_inactive_connection},
};
use actix::{Actor, Addr, AsyncContext, Context};
use uuid::Uuid;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::SocketAddrV4,
    str::FromStr,
};

pub struct TcpStreamsManager {
    pub streams: HashMap<SocketAddrV4, HashSet<Addr<WsSession>>>,
    pub streams_actors: HashMap<SocketAddrV4, Addr<TcpStreamActor>>,
    pub uuids_sockets: HashMap<Uuid, Option<String>>,
    pub uuids_users: HashMap<Uuid, i32>,
    pub latest_socket: Option<SocketAddrV4>,
    pub avail_map: HashMap<SocketAddrV4, Option<Addr<WsSession>>>,
    pub sockets: HashMap<SocketAddrV4, String>,
    pub inactive_sockets: VecDeque<Socket>,
    pub pgpool: actix_web::web::Data<AppState>,
}

impl TcpStreamsManager {
    pub async fn new(pgpool: actix_web::web::Data<AppState>) -> Result<Self, sqlx::Error> {
        let (sockets, latest_socket) = remove_inactive_connection(pgpool.clone()).await?;
        Ok(Self {
            streams: HashMap::new(),
            streams_actors: HashMap::new(),
            uuids_sockets: HashMap::new(),
            uuids_users: HashMap::new(),
            avail_map: HashMap::new(),
            inactive_sockets: VecDeque::new(),
            pgpool: pgpool.clone(),
            sockets,
            latest_socket,
        })
    }
    pub fn handle_message(&self, msg: SetMessage) {
        let addr = &msg.addr;
        for stream in &self.streams {
            if stream.1.contains(addr) {
                let socket = stream.0;
                let tcp_actor = self.streams_actors.get(socket).unwrap();
                let availability = self.avail_map.get(socket).unwrap();
                if let Some(wsocket) = availability {
                    if wsocket == &msg.addr {
                        tcp_actor.do_send(msg.clone());
                    }
                } else {
                    tcp_actor.do_send(msg.clone());
                }
            }
        }
    }
    pub fn post_middleware(&mut self, msg: MatrixReady, session: Addr<WsSession>) -> MatrixReady {
        self.avail_map.entry(msg.socket).or_insert(None);
        let availability = self.avail_map.get(&msg.socket).unwrap();

        let mut states = msg.states;
        states = attach_availability(states, availability, &session);

        MatrixReady {
            states,
            socket: msg.socket,
        }
    }
}

impl Actor for TcpStreamsManager {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let pgpool = self.pgpool.clone();
        let available_connetion = self.sockets.clone();
        let latest_socket = self.latest_socket.clone();
        let self_addr = ctx.address().clone();
        tokio::spawn(async move {
            let sockets = retrieve_sockets(&pgpool).await;
            if sockets.is_ok() {
                let sockets = sockets.unwrap();
                let mut unavailable_sockets: HashMap<SocketAddrV4, Socket> = HashMap::new();
                sockets.into_iter().for_each(|sock| {
                    unavailable_sockets.insert(SocketAddrV4::from_str(&sock.socket).unwrap(), sock);
                });
                available_connetion.iter().for_each(|s| {
                    unavailable_sockets.remove(s.0);
                });
                if latest_socket.is_some() {
                    unavailable_sockets.remove(&latest_socket.unwrap());
                }
                self_addr.do_send(UnavailableSockets {
                    sockets: unavailable_sockets.values().cloned().collect(),
                });
            }
        });
        let interval = ping_socket_settings::get_inactive_sockets_polling_interval();
        ctx.run_interval(interval, |act, ctx| {
            let inactive_sockets = act.inactive_sockets.clone();
            let pgpool = act.pgpool.clone();
            TcpStreamsManager::poll_sockets(inactive_sockets, ctx.address(),pgpool);
            
        });
    }
}
