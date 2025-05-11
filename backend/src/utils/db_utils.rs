use crate::configs::DatabaseEnv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = DatabaseEnv::get_db_url();
    let pgpool =PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("cannot connect to the database");

    sqlx::query("SET TIME ZONE 'Europe/Rome'")
        .execute(&pgpool)
        .await
        .expect("Failed to set timezone");

    pgpool
}
