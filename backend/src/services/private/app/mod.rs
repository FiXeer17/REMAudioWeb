use actix_web::web;

pub mod routes;
pub mod server;
pub mod session;
pub mod schemas;
use routes::app;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(app);
}
