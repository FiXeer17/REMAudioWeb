use crate::{db::establish_connection,services::create_user};
use actix_web::{web::Data, App, HttpServer};
use sqlx::{Pool, Postgres};
use actix_cors;


pub mod db;
pub mod services;
pub mod schemas;
pub mod hasher;
pub mod interfaces;


const SERVER_ADDR: &str= "0.0.0.0";
const SERVER_PORT: u16 = 8000;

pub struct AppState {
    pub db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{

    let pool = establish_connection().await;
    let _ = sqlx::migrate!("./migrations").run(&pool).await;
    println!("Server serving on: http://localhost:8000");
    HttpServer::new(move ||{
        let cors = actix_cors::Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(Data::new(AppState{db:pool.clone()}))
            .service(create_user)
    })
    .bind((SERVER_ADDR,SERVER_PORT)).expect(&format!("failed to run server on {}:{}",SERVER_ADDR,SERVER_PORT))
    .run()
    .await
}
