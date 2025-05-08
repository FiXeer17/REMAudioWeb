use backend::create_app;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use env_logger::{Builder, Env};
use log::LevelFilter;
use std::io::Write;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    Builder::from_env(Env::default().default_filter_or("info"))
    .format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] [{}] {}",
            Utc::now().to_rfc3339(),                     
            record.level(),                              
            record.target(),                             
            record.args()                                
        )
    })
    .filter_level(LevelFilter::Info)
    .init();
    create_app().await
}
