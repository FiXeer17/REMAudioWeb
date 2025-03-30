use actix_web::{
    get, web::{self}, HttpRequest, HttpResponse, Responder
};

use crate::services::{private::app::{messages::GetConnections, tcp_manager::tcp_manager::TcpStreamsManager} , public::root::schemas::ReturnSockets};

#[get("")]
pub async fn root(
    _req: HttpRequest,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
) -> impl Responder{
    let message = GetConnections{};
    match srv.send(message).await{
        Ok(sockets) =>{
            let return_mess =ReturnSockets::new(sockets);
            HttpResponse::Ok().json(return_mess)
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}