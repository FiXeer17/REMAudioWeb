use actix_web::web;
pub mod routes;
pub mod schemas;

use routes::socket;

pub fn router(cfg: &mut web::ServiceConfig){
    cfg.service(socket);
}