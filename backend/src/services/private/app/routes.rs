use crate::{
    services::private::app::{
        messages::{CheckSessionUUID, PendingConnections, RetrieveSocket, RetrieveUserFromUuid},
        schemas::SessionUUID,
        tcp_manager::tcp_manager::TcpStreamsManager,
        ws_session::session::WsSession,
    },
    utils::common::check_socket,
    AppState,
};
use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::utils::common::return_json_reason;
use actix_web_actors::ws;
use std::{net::SocketAddrV4, str::FromStr, time::Instant};
use uuid::Uuid;

#[get("/app")]
pub async fn app(
    req: HttpRequest,
    stream: web::Payload,
    pgpool: web::Data<AppState>,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
    uuid: web::Query<SessionUUID>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Err(_) = Uuid::from_str(&uuid.uuid) {
        return Ok(HttpResponse::Unauthorized().json(return_json_reason("Invalid uuid found")));
    }
    let uuid = Uuid::from_str(&uuid.uuid).unwrap();
    let checked = srv.send(CheckSessionUUID { uuid }).await;
    if let Err(e) = checked {
        return Ok(HttpResponse::InternalServerError().json(return_json_reason(&format!("{}", e))));
    }
    if let Ok(false) = checked {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    if let Ok(false) = srv.send(PendingConnections {}).await {
        return Ok(HttpResponse::NotFound().finish());
    }

    let socket = srv.send(RetrieveSocket { uuid }).await;
    if let Err(e) = socket {
        return Ok(HttpResponse::InternalServerError().json(return_json_reason(&format!("{}", e))));
    }
    let socket = socket.unwrap();
    let mut sockv4: Option<SocketAddrV4> = None;
    if socket.is_some() {
        sockv4 = check_socket(socket.unwrap()).unwrap();
    }

    let user_id = srv.send(RetrieveUserFromUuid { uuid }).await;
    if let Err(e) = user_id {
        return Ok(HttpResponse::InternalServerError().json(return_json_reason(&e.to_string())));
    }
    let user_id = user_id.unwrap();
    if user_id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let session = WsSession {
        hb: Instant::now(),
        srv: srv.get_ref().clone(),
        socket: sockv4,
        pgpool: pgpool.clone(),
        user_id: user_id.unwrap(),
    };
    ws::start(session, &req, stream)
}
