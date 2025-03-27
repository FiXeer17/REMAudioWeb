use crate::utils::db::establish_connection;
use actix::Actor;
use actix_cors;
use actix_web::{
    middleware::{from_fn, Logger, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use services::{
    private::{self, app::tcp_manager},
    public,
};
use sqlx::{Pool, Postgres};
use utils::auth_middleware::auth_middleware;

pub mod engine;
pub mod services;
pub mod utils;

pub const SERVER_ADDR: &str = "0.0.0.0";
pub const SERVER_PORT: u16 = 8000;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn crate_app() -> Result<(), std::io::Error> {
    env_logger::init();
    let pool = establish_connection().await; // create a connection with the database
    let _ = sqlx::migrate!("./migrations").run(&pool).await; // migrate 
    let server = tcp_manager::TcpStreamsManager::new().start(); // start tcp connections manager
    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
            

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
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
                    .service(
                        web::scope("/auth")
                            .wrap(from_fn(auth_middleware))
                            .configure(private::auth::router),
                    )
                    .configure(private::app::router)
            )
    })
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run()
    .await
}