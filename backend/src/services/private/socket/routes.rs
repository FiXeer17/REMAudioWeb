use crate::{services::private::{app::{messages::SetSocket, schemas::SessionUUID, tcp_manager::tcp_manager::TcpStreamsManager}, socket::schemas::SetSocketBody}, utils::common::{check_socket, return_json_reason}};
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;



#[post("")]
pub async fn socket(
    request_body: web::Json<SetSocketBody>,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
    uuid: web::Query<SessionUUID>,

) -> impl Responder{
    let socket = &request_body.socket;
    
    let socket = match check_socket(socket.to_string()){
        Ok(s) =>{
            let message = SetSocket{
                socket: socket.to_string(),
                uuid: uuid.uuid.clone()
            };
            let response = srv.send(message).await;
            if let Err(_) = response{
                return HttpResponse::InternalServerError().json(return_json_reason("couldn't set the socket"));
            }
            if let false = response.unwrap(){
                return HttpResponse::BadRequest().json(return_json_reason("invalid uuid"));
            }
            s.unwrap()
        },
        Err(e) => {
            return HttpResponse::BadRequest().json(return_json_reason(&format!("bad socket, {}",e.to_string())));
        }
    };

    
    
    let response = json!({"socket": socket});
    HttpResponse::Ok().json(response)
}