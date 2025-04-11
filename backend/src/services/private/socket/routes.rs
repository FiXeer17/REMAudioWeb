use std::str::FromStr;

use crate::{
    services::{private::{
        app::{
            messages::{GetConnections, RemoveSocket, RetrieveUserFromUuid, SetSocket},
            schemas::SessionUUID,
            tcp_manager::tcp_manager::TcpStreamsManager,
        },
        socket::{
            schemas::{RemoveSocketBody, SetSocketBody},
            utils::{check_in_connections, try_connection},
        },
    }, public::interfaces::retrieve_admin_from_id},
    utils::common::{check_socket, return_json_reason}, AppState,
};
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

#[post("/add")]
pub async fn add_socket(
    request_body: web::Json<SetSocketBody>,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
    pgpool: web::Data<AppState>,
    uuid: web::Query<SessionUUID>,
) -> impl Responder {
    let socket = &request_body.socket;
    let uuid_check = match Uuid::from_str(&uuid.uuid) {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::UnprocessableEntity().json(return_json_reason(&e.to_string())) 
    };
    match srv.send(RetrieveUserFromUuid{uuid:uuid_check}).await{
        Ok(Some(id)) => match retrieve_admin_from_id(&pgpool, id).await {
            Ok(true) => {println!("{id}")},
            Ok(false) => return HttpResponse::Unauthorized().finish(),
            Err(e)=> return HttpResponse::InternalServerError().json(return_json_reason(&e.to_string()))
            
        },
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(e) => return HttpResponse::InternalServerError().json(return_json_reason(&e.to_string()))
    };

    let (socket,socket_name):(String,String)  = match check_socket(socket.to_string()) {
        Ok(s) => {
            if let Some(sock) = s {
                let sockets = srv.send(GetConnections {}).await;
                if let Ok(connections) = sockets {
                    match check_in_connections(sock, connections) {
                        true => (),
                        false => {
                            if !try_connection(sock).await {
                                return HttpResponse::BadRequest().json(return_json_reason(
                                    &format!("{} doesn't respond.", sock),
                                ));
                            }
                        }
                    }
                }
            }
            let message = SetSocket {
                socket_name:request_body.socket_name.clone(),
                socket: socket.to_string(),
                uuid: uuid.uuid.clone(),
            };
            let response = srv.send(message).await;
            if let Err(_) = response {
                return HttpResponse::InternalServerError()
                    .json(return_json_reason("couldn't set the socket"));
            }
            if let false = response.unwrap() {
                return HttpResponse::BadRequest().json(return_json_reason("invalid uuid"));
            }
            (s.unwrap().to_string(),request_body.socket_name.clone())
            
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(return_json_reason(&format!(
                "bad socket, {}",
                e.to_string()
            )));
        }
    };

    let response = json!({"name":socket_name,"socket": socket});
    HttpResponse::Ok().json(response)
}



#[post("/remove")]
pub async fn remove_socket(
    request_body: web::Json<RemoveSocketBody>,
    srv: web::Data<actix::Addr<TcpStreamsManager>>,
    pgpool: web::Data<AppState>,
    uuid: web::Query<SessionUUID>,
) -> impl Responder {
    let socket = &request_body.socket;
    let uuid_check = match Uuid::from_str(&uuid.uuid) {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::UnprocessableEntity().json(return_json_reason(&e.to_string())) 
    };
    match srv.send(RetrieveUserFromUuid{uuid:uuid_check}).await{
        Ok(Some(id)) => match retrieve_admin_from_id(&pgpool, id).await {
            Ok(true) => {println!("{id}")},
            Ok(false) => return HttpResponse::Unauthorized().finish(),
            Err(e)=> return HttpResponse::InternalServerError().json(return_json_reason(&e.to_string()))
            
        },
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(e) => return HttpResponse::InternalServerError().json(return_json_reason(&e.to_string()))
    };

    
    let socket = match check_socket(socket.to_string()) {
        Ok(s) => {
            let sockets = srv.send(GetConnections {}).await;
            if let Ok(connections) = sockets{
                match check_in_connections(s.unwrap(), connections){
                    true => (),
                    false => return HttpResponse::NotFound().finish()
                }
            }
            let message = RemoveSocket {
                socket:s.unwrap(),
                uuid: uuid.uuid.clone(),
            };
            srv.do_send(message);
            s.unwrap()
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(return_json_reason(&format!(
                "bad socket, {}",
                e.to_string()
            )));
        }
    };

    let response = json!({"socket": socket});
    HttpResponse::Ok().json(response)
}

