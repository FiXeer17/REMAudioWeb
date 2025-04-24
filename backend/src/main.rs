use backend::crate_app;
use sqlx::{Pool, Postgres};


pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    crate_app().await
}
