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
            utils::{try_connection, Device},
        },
    }, public::{interfaces::{remove_socket_in_db, retrieve_admin_from_id}, schemas::IsContainedExt}},
    utils::common::{check_socket, toast}, AppState,
};
use actix_web::{post, web, HttpResponse, Responder};
use log::{info,error};
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
        Err(e) => return HttpResponse::UnprocessableEntity().json(toast(&e.to_string())) 
    };
    match srv.send(RetrieveUserFromUuid{uuid:uuid_check}).await{
        Ok(Some(id)) => match retrieve_admin_from_id(&pgpool, id).await {
            Ok(true) => (),
            Ok(false) => return HttpResponse::Unauthorized().finish(),
            Err(e)=> return HttpResponse::InternalServerError().json(toast(&e.to_string()))
            
        },
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(e) => return HttpResponse::InternalServerError().json(toast(&e.to_string()))
    };
    let message: SetSocket;
    let (socket,socket_name):(String,String)  = match check_socket(socket.to_string()) {
        Ok(s) => {
            if let Some(sock) = s {
                let sockets = srv.send(GetConnections {}).await;
                if let Ok(connections) = sockets {
                    if let Some(connections) = connections{
                        match connections.socket_is_contained(&sock.to_string()) {
                            Some(_) => (),
                            None => {
                                if !try_connection(sock).await {
                                    return HttpResponse::BadRequest().json(toast(
                                        &format!("{} doesn't respond.", sock),
                                    ));
                                }
                            }
                        }
                    }
                    
                }
                if let Err(()) = Device::from_str(&request_body.device_type){
                    return HttpResponse::BadRequest().json(toast("invalid device type"));
                }

                message = SetSocket {
                    socket_name:request_body.socket_name.clone(),
                    socket: sock.to_string(),
                    uuid: uuid.uuid.clone(),
                    device:request_body.device_type.clone()
                };
                let Ok(response) = srv.send(message).await else {
                    return HttpResponse::InternalServerError()
                        .json(toast("couldn't set the socket"));
                };
                if let false = response {
                    return HttpResponse::BadRequest().json(toast("invalid uuid"));
                }
            }
         

            (s.unwrap().to_string(),request_body.socket_name.clone())
            
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(toast(&format!(
                "bad socket, {}",
                e.to_string()
            )));
        }
    };

    let response = json!({"name":socket_name,"socket": socket,"device":request_body.device_type});
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
        Err(e) => return HttpResponse::UnprocessableEntity().json(toast(&e.to_string())) 
    };
    match srv.send(RetrieveUserFromUuid{uuid:uuid_check}).await{
        Ok(Some(id)) => match retrieve_admin_from_id(&pgpool, id).await {
            Ok(true) => (),
            Ok(false) => return HttpResponse::Unauthorized().finish(),
            Err(e)=> return HttpResponse::InternalServerError().json(toast(&e.to_string()))
            
        },
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(e) => return HttpResponse::InternalServerError().json(toast(&e.to_string()))
    };

    
    match check_socket(socket.to_string()) {
        Ok(s) => {
            if let Err(e) = remove_socket_in_db(&pgpool, s.unwrap()).await{
                error!("Cannot remove socket from database, error:\n{}",e.to_string());
                return HttpResponse::InternalServerError().finish();
            }
            info!("Socket: {} removed from database.",s.unwrap());
            let sockets = srv.send(GetConnections {}).await;
            if let Ok(connections) = sockets{
                if let Some(connections) = connections{
                    match connections.socket_is_contained(&s.unwrap().to_string()){
                        Some(_) => (),
                        None => {  
                            return HttpResponse::Ok().json(json!({"socket": s.unwrap().to_string()}));
                        }
                    }
                }
                
            }
            
            let message = RemoveSocket {
                socket:s.unwrap(),
            };
            srv.do_send(message);
            s.unwrap();
            
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(toast(&format!(
                "bad socket, {}",
                e.to_string()
            )));
        }
    };

    let response = json!({"socket": request_body.socket});
    HttpResponse::Ok().json(response)
}

