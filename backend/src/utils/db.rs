use crate::utils::env_dns::Env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = Env::get_db_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("cannot connect to the database")
}
