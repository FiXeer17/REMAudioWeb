use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::services::private::app::server::WsSession;
use std::time::Instant;

#[get("/app")]
pub async fn app(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse,actix_web::Error>{
    ws::start(WsSession{hb: Instant::now()}, &req, stream)
}
