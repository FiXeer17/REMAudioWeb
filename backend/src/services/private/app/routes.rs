use crate::{
    services::private::app::{
        messages::{CheckSessionUUID, RetrieveSocket, RetrieveUserFromUuid},
        schemas::SessionUUID,
        tcp_manager::tcp_manager::TcpStreamsManager,
        ws_session::session::WsSession,
    },
    AppState,
};
use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::utils::common::toast;
use actix_web_actors::ws;
use std::{str::FromStr, time::Instant};
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
        return Ok(HttpResponse::Unauthorized().json(toast("Invalid uuid found")));
    }
    let Ok(uuid) = Uuid::from_str(&uuid.uuid) else {return Ok(HttpResponse::BadRequest().json(toast("Invalid uuid")))};
    let checked = srv.send(CheckSessionUUID { uuid }).await;
    if let Err(e) = checked {
        return Ok(HttpResponse::InternalServerError().json(toast(&format!("{}", e))));
    }
    if let Ok(false) = checked {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let socket = srv.send(RetrieveSocket { uuid }).await;
    if let Err(e) = socket {
        return Ok(HttpResponse::InternalServerError().json(toast(&format!("{}", e))));
    }

    let Ok(user_id) = srv.send(RetrieveUserFromUuid { uuid }).await else {return Ok(HttpResponse::InternalServerError().finish());};
    let Some(user_id) = user_id else {return Ok(HttpResponse::Unauthorized().finish());};

    let session = WsSession {
        hb: Instant::now(),
        srv: srv.get_ref().clone(),
        socket: None,
        pgpool: pgpool.clone(),
        user_id: user_id,
    };
    ws::start(session, &req, stream)
}
