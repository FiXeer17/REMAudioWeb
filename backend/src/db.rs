use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::env_dns::Env;



pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = Env::get_vars().get_db_url();
    println!("{db_url}");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.expect("cannot connect to the database")
}
