pub mod routes;
pub mod schemas;

use actix_web::web;
use routes::register;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}
