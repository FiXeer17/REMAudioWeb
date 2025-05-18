use actix_web::web;
use routes::stream;

pub mod messages;
pub mod routes;
pub mod schemas;
pub mod streams_manager;
pub mod stream_handler;
pub mod utils;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(stream);
}
