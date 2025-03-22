use actix_web::web;
pub mod routes;
use routes::auth;

pub fn router(cfg: &mut web::ServiceConfig){
    cfg.service(auth);
}