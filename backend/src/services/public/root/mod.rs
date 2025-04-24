use actix_web::web::ServiceConfig;

pub mod routes;
pub mod schemas;

use routes::root;

pub fn router(cfg: &mut ServiceConfig){
    cfg.service(root);
}