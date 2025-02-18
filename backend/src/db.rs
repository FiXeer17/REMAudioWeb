use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::{dotenv,from_filename};



pub async fn establish_connection() -> Pool<Postgres> {
    from_filename(".env.local").ok();
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    println!("{db_url}");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.expect("cannot connect to the database")
}
