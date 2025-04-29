use std::{collections::HashMap, net::SocketAddrV4, str::FromStr};

use actix_web::{
    get,
    web::{self},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    services::{
        private::app::{
            messages::{GetConnections, GetLatestConnection},
            tcp_manager::tcp_manager::TcpStreamsManager,
        },
        public::{interfaces::retrieve_sockets, root::schemas::ReturnSockets},
    },
    utils::common::toast,
    AppState,
};

#[get("")]
pub async fn root(
    _req: HttpRequest,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
) -> impl Responder {
    let message = GetConnections {};
    match srv.send(message).await {
        Ok(sockets) => {
            let latest_socket = srv.send(GetLatestConnection {}).await;
            if let Err(_) = latest_socket {
                return HttpResponse::InternalServerError()
                    .json(toast("cannot read latest connection."));
            }
            let return_mess = ReturnSockets::new(sockets, latest_socket.unwrap());
            HttpResponse::Ok().json(return_mess)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/get_all")]
pub async fn get_all(_request: HttpRequest, pgpool: web::Data<AppState>) -> impl Responder {
    let Ok(db_sockets) = retrieve_sockets(&pgpool).await else {
        return HttpResponse::InternalServerError().finish();
    };
    let mut sockets: HashMap<SocketAddrV4, String> = HashMap::new();
    let mut latest_socket: HashMap<SocketAddrV4, String> = HashMap::new();
    for socket in db_sockets {
        if socket.latest {
            latest_socket.clear();
            latest_socket.insert(
                SocketAddrV4::from_str(&socket.socket).unwrap(),
                socket.socket_name.clone(),
            );
        }
        sockets.insert(
            SocketAddrV4::from_str(&socket.socket).unwrap(),
            socket.socket_name,
        );
    }
    
    let (return_sockets, return_latest_socket): (
        Option<HashMap<SocketAddrV4, String>>,
        Option<HashMap<SocketAddrV4, String>>,
    );
    match sockets.is_empty(){
        true => {return_sockets = None},
        false => {return_sockets = Some(sockets)}
    }
    match latest_socket.is_empty(){
        true => {return_latest_socket = None },
        false => {return_latest_socket = Some(latest_socket)}
    }
    let return_mess = ReturnSockets::new(return_sockets, return_latest_socket);
    HttpResponse::Ok().json(return_mess)
}
