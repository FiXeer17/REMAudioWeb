use actix_web::{
    get, web::{self}, HttpRequest, HttpResponse, Responder
};

use crate::{services::{private::app::{messages::{GetConnections, GetLatestConnection}, tcp_manager::tcp_manager::TcpStreamsManager} , public::root::schemas::ReturnSockets}, utils::common::return_json_reason};

#[get("")]
pub async fn root(
    _req: HttpRequest,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
) -> impl Responder{
    let message = GetConnections{};
    match srv.send(message).await{
        Ok(sockets) =>{
            let latest_socket = srv.send(GetLatestConnection{}).await;
            if let Err(_) = latest_socket{
                return HttpResponse::InternalServerError().json(return_json_reason("cannot read latest connection."));
            } 
            let return_mess =ReturnSockets::new(sockets,latest_socket.unwrap());
            HttpResponse::Ok().json(return_mess)
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}