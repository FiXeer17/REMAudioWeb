use actix_web::web;
pub mod routes;
pub mod schemas;
pub mod utils;
pub mod configs;

use routes::socket;

pub fn router(cfg: &mut web::ServiceConfig){
    cfg.service(socket);
}