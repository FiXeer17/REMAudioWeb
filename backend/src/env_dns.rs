use dotenv::{dotenv, from_filename};

pub const DATABASE_URL: &str = "DATABASE_URL";
pub const JWT_SECRET: &str = "JWT_SECRET";

pub struct Env {
    database_url: String,
    jwt_secret: String,
}

impl Env {
    pub fn get_vars() -> Self {
        from_filename(".env.local").ok();
        dotenv().ok();

        let database_url: String =
            std::env::var(DATABASE_URL).expect("failed to retrive database url.");
        let jwt_secret: String = std::env::var(JWT_SECRET).expect("failed to retrive jwt secret.");
        Env {
            database_url,
            jwt_secret,
        }
    }

    pub fn get_db_url(&self) -> String {
        Env::get_vars().database_url
    }

    pub fn get_jwt_secret(&self) -> String {
        Env::get_vars().jwt_secret
    }
}
