use crate::services::private::app::{server::WsServer, session::WsSession};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

#[get("/app")]
pub async fn app(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<actix::Addr<WsServer>>
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(
        WsSession {
            hb: Instant::now(),
            srv: srv.get_ref().clone()
        },
        &req,
        stream,
    )
}
