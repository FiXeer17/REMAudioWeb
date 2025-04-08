use crate::{services::private::app::tcp_manager::tcp_manager::TcpStreamsManager, utils::jwt_utils::bearertkn_to_id};
use actix_web::{get, http::header::AUTHORIZATION, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use super::super::app::messages::SessionOpened;

#[get("")]
pub async fn auth(
    req: HttpRequest,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
) -> impl Responder {
    let jwt_token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    let user_id = bearertkn_to_id(jwt_token);
    let uuid = srv.send(SessionOpened {socket:None,user_id}).await.unwrap();
    let response = json!({ "uuid": uuid});
    return HttpResponse::Ok().json(response);
}
