use crate::services::private::app::{messages::CheckSessionUUID, schemas::SessionUUID, tcp_manager::tcp_manager::TcpStreamsManager, ws_session::session::WsSession, };
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::utils::common::return_json_reason;
use std::{str::FromStr, time::Instant};

#[get("/app")]
pub async fn app(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
    uuid: web::Query<SessionUUID>
) -> Result<HttpResponse, actix_web::Error> {

    if let Err(_) = Uuid::from_str(&uuid.uuid){
        return Ok(HttpResponse::Unauthorized().json(return_json_reason("Invalid uuid found")));
    }
    let uuid = Uuid::from_str(&uuid.uuid).unwrap();
    let checked = srv.send(CheckSessionUUID{uuid}).await;
    if let Err(e) = checked{
        return Ok(HttpResponse::InternalServerError().json(return_json_reason(&format!("{}",e))));
    }
    if let Ok(false) = checked{
        return  Ok(HttpResponse::Unauthorized().finish());
    }

    ws::start(
        WsSession {
            hb: Instant::now(),
            srv: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
