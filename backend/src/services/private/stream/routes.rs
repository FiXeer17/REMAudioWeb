use std::{net::SocketAddrV4, str::FromStr};

use crate::{
    services::private::{
        socket::utils::try_connection,
        stream::{messages::Connect, schemas::B64Address, utils::handle_stream},
    },
    utils::common::toast,
};
use actix::Addr;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use base64::{prelude::BASE64_URL_SAFE, Engine};

use super::streams_manager::streams_manager::StreamManager;

#[get("")]
pub async fn stream(
    _: HttpRequest,
    stream_srv: web::Data<Addr<StreamManager>>,
    rtsp_socket: web::Query<B64Address>,
) -> impl Responder {
    let rtsp_socket = rtsp_socket.a.clone();
    let Ok(rtsp_socket) = BASE64_URL_SAFE.decode(rtsp_socket) else {
        return HttpResponse::BadRequest().json(toast("Invalid base64 query param"));
    };
    let Ok(rtsp_string) = String::from_utf8(rtsp_socket) else {
        return HttpResponse::BadRequest().json(toast("Invalid utf8"));
    };
    let Ok(rtsp_socket) = SocketAddrV4::from_str(&rtsp_string) else {
        return HttpResponse::BadRequest().json(toast("Invalid socket"));
    };
    if !try_connection(rtsp_socket).await {
        return HttpResponse::InternalServerError().json(toast("rtsp stream is not available"));
    }
    let session = stream_srv
        .send(Connect {
            socket: rtsp_socket,
        })
        .await;
    match session {
        Ok(res) => match res {
            Ok(tx) => return handle_stream(tx),
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}
