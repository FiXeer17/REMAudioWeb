use actix_web::web;
pub mod routes;
pub mod schemas;
pub mod utils;

use routes::{add_socket,remove_socket};

pub fn router(cfg: &mut web::ServiceConfig){
    cfg.service(add_socket)
        .service(remove_socket);
}