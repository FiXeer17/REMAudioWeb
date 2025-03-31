use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::services::private::app::tcp_manager::tcp_manager::TcpStreamsManager;

use super::super::app::messages::SessionOpened;

#[get("")]
pub async fn auth(
    _req: HttpRequest,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
) -> impl Responder {
    let uuid = srv.send(SessionOpened{}).await.unwrap(); 
    let response = json!({ "uuid": uuid});
    return HttpResponse::Ok().json(response);
}
