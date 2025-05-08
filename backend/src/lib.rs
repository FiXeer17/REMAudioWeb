use crate::utils::db_utils::establish_connection;
use actix::Actor;
use actix_cors;
use actix_web::{
    middleware::{from_fn, Logger, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use services::{
    private::{self, app::tcp_manager::tcp_manager::TcpStreamsManager},
    public::{self, interfaces::insert_default_user},
};
use sqlx::{Pool, Postgres};
use utils::auth_middleware::auth_middleware;

pub mod engines;
pub mod services;
pub mod utils;
pub mod configs;


pub const SERVER_ADDR: &str = "0.0.0.0";
pub const SERVER_PORT: u16 = 8000;

#[derive(Clone,Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn create_app() -> Result<(), std::io::Error> {
    let pool = establish_connection().await; // create a connection with the database
    let _ = sqlx::migrate!("./migrations").run(&pool).await.unwrap(); // migrate
    let app_state = AppState { db: pool.clone() };
    let dbdata =Data::new(app_state.clone());
    insert_default_user(&app_state).await.unwrap();
    let server = TcpStreamsManager::new(dbdata.clone()).await.expect("cannot start tcp manager...").start(); // start tcp connections manager
    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .app_data(dbdata.clone())
            .app_data(Data::new(server.clone()))
            .service(
                web::scope("/api")
                            .configure(public::root::router)
                            .service(
                    web::scope("/auth")
                        .configure(public::signin::router),
                ),
            )
            .service(
                web::scope("/ws")
                    .service(
                        web::scope("/auth")
                            .wrap(from_fn(auth_middleware))
                            .configure(private::auth::router)    
                    ).service(
                        web::scope("/socket")
                        .configure(private::socket::router)
                    )
                    .configure(private::app::router),
            )
    })
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run()
    .await
}
