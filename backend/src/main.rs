use crate::utils::db::establish_connection;
use actix_cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use services::public;
use sqlx::{Pool, Postgres};

pub mod services;
pub mod utils;

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 8000;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let pool = establish_connection().await;
    let _ = sqlx::migrate!("./migrations").run(&pool).await;
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                web::scope("/api").service(
                    web::scope("/auth")
                        .configure(public::register::router)
                        .configure(public::login::router),
                ),
            )
    })
    .bind((SERVER_ADDR, SERVER_PORT))
    .expect(&format!(
        "failed to run server on {}:{}",
        SERVER_ADDR, SERVER_PORT
    ))
    .run()
    .await
}
