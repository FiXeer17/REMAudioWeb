use std::collections::HashSet;

use actix_web::{
    get,
    web::{self},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    services::{
        private::app::{messages::GetConnections, tcp_manager::tcp_manager::TcpStreamsManager},
        public::{interfaces::retrieve_sockets, root::schemas::ReturnSockets, schemas::Socket},
    },
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
            let return_mess = ReturnSockets::new(sockets);
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
    let db_sockets: Option<HashSet<Socket>> = match db_sockets.is_empty() {
        true => None,
        false => Some(db_sockets.into_iter().collect()),
    };
    let return_mess = ReturnSockets::new(db_sockets);
    HttpResponse::Ok().json(return_mess)
}
