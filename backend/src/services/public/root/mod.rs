use actix_web::web::ServiceConfig;

pub mod routes;
pub mod schemas;

use routes::{get_all, root};

pub fn router(cfg: &mut ServiceConfig){
    cfg.service(root);
    cfg.service(get_all);
}