use crate::{
    services::{
        private::app::{
            messages::{CheckSessionUUID, GetConnections, RetrieveSocket, RetrieveUserFromUuid},
            schemas::SessionUUID,
            tcp_manager::tcp_manager::TcpStreamsManager,
            ws_session::session::WsSession,
        },
        public::{interfaces, utils::SRC},
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
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
    pgpool: web::Data<AppState>,
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

    if let Ok(None) = srv.send(GetConnections{}).await{
        return Ok(HttpResponse::NotFound().finish())
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
    let i_channels = interfaces::retrieve_channels(&pgpool, user_id, SRC::INPUT)
        .await
        .unwrap()
        .unwrap();
    let o_channels = interfaces::retrieve_channels(&pgpool, user_id, SRC::OUTPUT)
        .await
        .unwrap()
        .unwrap();

    ws::start(
        WsSession {
            hb: Instant::now(),
            srv: srv.get_ref().clone(),
            socket: sockv4,
            user_id,
            i_channels,
            o_channels,
        },
        &req,
        stream,
    )
}
