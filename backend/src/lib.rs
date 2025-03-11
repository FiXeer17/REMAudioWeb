use crate::utils::db::establish_connection;
use actix::Actor;
use actix_cors;
use actix_web::{
    middleware::{from_fn, Logger},
    web::{self, Data},
    App, HttpServer,
};
use services::{private, public,private::app::server};
use sqlx::{Pool, Postgres};
use utils::auth_middleware::auth_middleware;


pub mod services;
pub mod utils;
pub mod engine;

pub const SERVER_ADDR: &str = "0.0.0.0";
pub const SERVER_PORT: u16 = 8000;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn crate_app() -> Result<(), std::io::Error> {
    env_logger::init();
    let pool = establish_connection().await;
    let _ = sqlx::migrate!("./migrations").run(&pool).await;
    let server = server::WsServer::new().start();
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .app_data(Data::new(server.clone()))
            .service(
                web::scope("/api").service(
                    web::scope("/auth")
                        .configure(public::register::router)
                        .configure(public::signin::router),
                ),
            )
            .service(
                web::scope("/ws")
                    .wrap(from_fn(auth_middleware))
                    .configure(private::app::router),
            )
    })
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run()
    .await
}
