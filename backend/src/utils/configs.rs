use dotenv::{dotenv, from_filename};


 //VARIABLES NAMES IN .env FILE:
pub const JWT_SECRET: &str = "JWT_SECRET";
pub const DATABASE_NAME: &str = "POSTGRES_DB";
pub const DATABASE_PASSWORD: &str = "POSTGRES_PASSWORD";
pub const DATABASE_USER: &str = "POSTGRES_USER";
pub const DEFAULT_SOCKET: &str = "DEFAULT_SOCKET";
pub const DEFAULT_ADMIN: &str = "DEFAULT_ADMIN";
pub const DEFAULT_ADMIN_PASSWORD: &str = "DEFAULT_ADMIN_PASSWORD";

#[allow(dead_code)]
pub struct Env {
    database_url: String,
    database_name: String,
    database_user: String,
    database_password: String,
    jwt_secret: String,
    default_socket: String,
    default_admin: String,
    default_admin_password: String
}

impl Env {
    pub fn get_vars() -> Self {
        from_filename(".env.local").ok();
        dotenv().ok();

        let jwt_secret: String = std::env::var(JWT_SECRET).expect("failed to retrive jwt secret.");
        let database_name = std::env::var(DATABASE_NAME).expect("failed to retrieve database name");
        let database_user =
            std::env::var(DATABASE_USER).expect("failed to retrieve database username");
        let database_password =
            std::env::var(DATABASE_PASSWORD).expect("failed to retrieve database password");
        let database_url = format!(
            "postgresql://{}:{}@db:5432/{}",
            database_user, database_password, database_name
        );
        let default_socket =
            std::env::var(DEFAULT_SOCKET).expect("failed to retrieve default socket");
        let default_admin =
            std::env::var(DEFAULT_ADMIN).expect("failed to retrieve default admin user");
        let default_admin_password =
            std::env::var(DEFAULT_ADMIN_PASSWORD).expect("failed to retrieve default admin user password");
        Env {
            database_url,
            database_name,
            database_user,
            database_password,
            jwt_secret,
            default_socket,
            default_admin,
            default_admin_password,
        }
    }

    pub fn get_db_url() -> String {
        Env::get_vars().database_url
    }
    pub fn get_jwt_secret() -> String {
        Env::get_vars().jwt_secret
    }
    pub fn get_db_name() -> String {
        Env::get_vars().database_name
    }
    pub fn get_default_socket() -> String {
        Env::get_vars().default_socket
    }
    pub fn get_default_admin() -> String {
        Env::get_vars().default_admin
    }
    pub fn get_default_admin_password() -> String {
        Env::get_vars().default_admin_password
    }
}
