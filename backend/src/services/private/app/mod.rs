use actix_web::web;
pub mod routes;
pub mod messages;
pub mod schemas;
pub mod tcp_manager;
pub mod tcp_handler;
pub mod ws_session;
pub mod utils;
use routes::app;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(app);
}
