use std::time::Duration;

use dotenv::{dotenv, from_filename};

use crate::services::private::app::{tcp_handler, ws_session};
use crate::services::private::socket;

//VARIABLES NAMES IN .env FILE:

//DB
pub const JWT_SECRET: &str = "JWT_SECRET";
pub const DATABASE_NAME: &str = "POSTGRES_DB";
pub const DATABASE_PASSWORD: &str = "POSTGRES_PASSWORD";
pub const DATABASE_USER: &str = "POSTGRES_USER";
pub const DEFAULT_ADMIN: &str = "DEFAULT_ADMIN";
pub const DEFAULT_ADMIN_PASSWORD: &str = "DEFAULT_ADMIN_PASSWORD";
pub const DEFAULT_USER: &str = "DEFAULT_USER";
pub const DEFAULT_USER_PASSWORD: &str = "DEFAULT_USER_PASSWORD";
//MATRIX CHANNEL CONFIGS
pub const CHANNEL_DEFAULT_PREFIX: &str = "CHANNEL_DEFAULT_PREFIX";
pub const I_CHANNEL_NUMBER: &str = "I_CHANNEL_NUMBER";
pub const O_CHANNEL_NUMBER: &str = "O_CHANNEL_NUMBER";
pub const DEFAULT_VISIBILITY: &str = "DEFAULT_VISIBILITY";

//CONNECTIVITY
pub const DEFAULT_SOCKET: &str = "DEFAULT_SOCKET";
//TCP
pub const COMMAND_DELAY: &str = "COMMAND_DELAY";
pub const RECONNECT_DELAY: &str = "RECONNECT_DELAY";
pub const READ_TIMEOUT: &str = "READ_TIMEOUT";
pub const CONNECTION_TIMEOUT: &str = "CONNECTION_TIMEOUT_TIME";
pub const INACTIVITY_TIMEOUT: &str = "INACTIVITY_TIMEOUT_TIME";
pub const MAX_RETRIES: &str = "MAX_RETRIES";
//PING
pub const PING_SOCKET_TIMEOUT: &str = "PING_SOCKET_TIMEOUT";
pub const PING_SOCKET_MAX_RETRIES: &str = "PING_SOCKET_MAX_RETRIES";
//WEBSOCKET
pub const HEARTBEAT_INTERVAL: &str = "HEARTBEAT_INTERVAL";
pub const CLIENT_TIMEOUT: &str = "CLIENT_TIMEOUT";

#[allow(dead_code, unused_variables)]

pub struct DatabaseEnv {
    database_url: String,
    database_name: String,
    database_user: String,
    database_password: String,
    default_admin: String,
    default_admin_password: String,
    default_user: String,
    default_user_password: String,
    jwt_secret: String,
}

pub struct ChannelsEnv {
    channel_default_prefix: String,
    i_channel_number: u32,
    o_channel_number: u32,
    default_visibility: bool,
}
pub struct ComunicationEnv {
    command_delay: Duration,
    reconnect_delay: Duration,
    read_timeout: Duration,
    connection_timeout: Duration,
    inactivity_timeout: Duration,
    max_retries: u8,
}
pub struct WebsocketEnv {
    heartbeat_interval: Duration,
    client_timeout: Duration,
}
pub struct PingEnv {
    ping_socket_timeout: Duration,
    ping_socket_max_retries: u8,
}

#[allow(dead_code, unused_variables)]
pub struct Env {
    pub database_settings: DatabaseEnv,
    pub tcp_comunication_settings: ComunicationEnv,
    pub websocket_settings: WebsocketEnv,
    pub channels_settings:ChannelsEnv
}
#[allow(dead_code, unused_variables)]
impl DatabaseEnv {
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
        let default_admin =
            std::env::var(DEFAULT_ADMIN).expect("failed to retrieve default admin user");
        let default_admin_password = std::env::var(DEFAULT_ADMIN_PASSWORD)
            .expect("failed to retrieve default admin user password");

        let default_user =
            std::env::var(DEFAULT_USER).expect("failed to retrieve default user username");
        let default_user_password =
            std::env::var(DEFAULT_USER_PASSWORD).expect("failed to retrieve default user password");
        DatabaseEnv {
            database_url,
            database_name,
            database_user,
            database_password,
            default_admin,
            default_admin_password,
            default_user,
            default_user_password,
            jwt_secret,
        }
    }
    pub fn get_db_url() -> String {
        DatabaseEnv::get_vars().database_url
    }
    pub fn get_jwt_secret() -> String {
        DatabaseEnv::get_vars().jwt_secret
    }
    pub fn get_db_name() -> String {
        DatabaseEnv::get_vars().database_name
    }
    pub fn get_default_admin() -> String {
        DatabaseEnv::get_vars().default_admin
    }
    pub fn get_default_admin_password() -> String {
        DatabaseEnv::get_vars().default_admin_password
    }
    pub fn get_default_user() -> String {
        DatabaseEnv::get_vars().default_user
    }
    pub fn get_default_user_password() -> String {
        DatabaseEnv::get_vars().default_user_password
    }
}

impl ChannelsEnv {
    pub fn get_vars() -> Self {
        let channel_default_prefix = std::env::var(CHANNEL_DEFAULT_PREFIX)
            .expect("failed to retrieve CHANNEL_DEFAULT_PREFIX");
        let i_channel_number = std::env::var(I_CHANNEL_NUMBER)
            .expect("failed to retrieve I_CHANNEL_NUMBER")
            .parse::<u32>()
            .expect("I_CHANNEL_NUMBER expected as a positive integer");
        let o_channel_number = std::env::var(O_CHANNEL_NUMBER)
            .expect("failed to retrieve O_CHANNEL_NUMBER")
            .parse::<u32>()
            .expect("O_CHANNEL_NUMBER expected as a positive integer");
        let default_visibility = std::env::var(DEFAULT_VISIBILITY)
            .expect("failed to retrieve DEFAULT_VISIBILITY")
            .parse::<bool>()
            .expect("DEFAULT_VISIBILITY expected as a boolean");
        ChannelsEnv {
            channel_default_prefix,
            i_channel_number,
            o_channel_number,
            default_visibility,
        }
    }
    pub fn get_channel_default_prefix()->String{
        ChannelsEnv::get_vars().channel_default_prefix
    }
    pub fn get_i_channel_number()->u32{
        ChannelsEnv::get_vars().i_channel_number
    }
    pub fn get_o_channel_number()->u32{
        ChannelsEnv::get_vars().o_channel_number
    }
    pub fn get_default_visibility()->bool{
        ChannelsEnv::get_vars().default_visibility
    }
}

impl ComunicationEnv {
    pub fn get_vars() -> Self {
        from_filename(".env.local").ok();
        dotenv().ok();
        let command_delay = std::env::var(COMMAND_DELAY)
            .unwrap_or(tcp_handler::configs::COMMAND_DELAY.to_string())
            .parse::<u64>()
            .expect(" COMMAND_DELAY expected as a positive integer");
        let reconnect_delay = std::env::var(RECONNECT_DELAY)
            .unwrap_or(tcp_handler::configs::RECONNECT_DELAY.to_string())
            .parse::<u64>()
            .expect(" RECONNECT_DELAY expected as a positive integer");
        let read_timeout = std::env::var(READ_TIMEOUT)
            .unwrap_or(tcp_handler::configs::READ_TIMEOUT.to_string())
            .parse::<u64>()
            .expect(" READ_TIMEOUT expected as a positive integer");
        let connection_timeout = std::env::var(CONNECTION_TIMEOUT)
            .unwrap_or(tcp_handler::configs::CONNECTION_TIMEOUT.to_string())
            .parse::<u64>()
            .expect(" READ_TIMEOUT expected as a positive integer");
        let inactivity_timeout = std::env::var(INACTIVITY_TIMEOUT)
            .unwrap_or(tcp_handler::configs::INACTIVITY_TIMEOUT.to_string())
            .parse::<u64>()
            .expect(" INACTIVITY_TIMEOUT_TIME expected as a positive integer");
        let max_retries = std::env::var(MAX_RETRIES)
            .unwrap_or(tcp_handler::configs::MAX_RETRIES.to_string())
            .parse::<u8>()
            .expect(" MAX_RETRIES expected as a positive integer");

        let command_delay = Duration::from_millis(command_delay);
        let reconnect_delay = Duration::from_millis(reconnect_delay);
        let read_timeout = Duration::from_millis(read_timeout);
        let connection_timeout = Duration::from_millis(connection_timeout);
        let inactivity_timeout = Duration::from_millis(inactivity_timeout);

        ComunicationEnv {
            command_delay,
            reconnect_delay,
            read_timeout,
            connection_timeout,
            inactivity_timeout,
            max_retries,
        }
    }
    pub fn get_command_delay() -> Duration {
        ComunicationEnv::get_vars().command_delay
    }
    pub fn get_reconnect_delay() -> Duration {
        ComunicationEnv::get_vars().reconnect_delay
    }
    pub fn get_read_timeout() -> Duration {
        ComunicationEnv::get_vars().read_timeout
    }
    pub fn get_connection_timeout() -> Duration {
        ComunicationEnv::get_vars().connection_timeout
    }
    pub fn get_inactivity_timeout() -> Duration {
        ComunicationEnv::get_vars().inactivity_timeout
    }
    pub fn get_max_retries() -> u8 {
        ComunicationEnv::get_vars().max_retries
    }
}

impl WebsocketEnv {
    pub fn get_vars() -> Self {
        from_filename(".env.local").ok();
        dotenv().ok();

        let heartbeat_interval = std::env::var(HEARTBEAT_INTERVAL)
            .unwrap_or(ws_session::configs::HEARTBEAT_INTERVAL.to_string())
            .parse::<u64>()
            .expect(" HEARTBEAT_INTERVAL expected as a positive integer");
        let client_timeout = std::env::var(CLIENT_TIMEOUT)
            .unwrap_or(ws_session::configs::CLIENT_TIMEOUT.to_string())
            .parse::<u64>()
            .expect(" CLIENT_TIMEOUT expected as a positive integer");

        let heartbeat_interval = Duration::from_millis(heartbeat_interval);
        let client_timeout = Duration::from_millis(client_timeout);
        WebsocketEnv {
            heartbeat_interval,
            client_timeout,
        }
    }
    pub fn get_heartbeat_interval() -> Duration {
        WebsocketEnv::get_vars().heartbeat_interval
    }
    pub fn get_client_timeout() -> Duration {
        WebsocketEnv::get_vars().client_timeout
    }
}

impl PingEnv {
    pub fn get_vars() -> Self {
        from_filename(".env.local").ok();
        dotenv().ok();

        let ping_socket_timeout = std::env::var(PING_SOCKET_TIMEOUT)
            .unwrap_or(socket::configs::PING_SOCKET_TIMEOUT.to_string())
            .parse::<u64>()
            .expect("PING_SOCKET_TIMEOUT expected as a positive integer.");
        let ping_socket_max_retries: u8 = std::env::var(PING_SOCKET_MAX_RETRIES)
            .unwrap_or(socket::configs::PING_SOCKET_MAX_RETRIES.to_string())
            .parse::<u8>()
            .expect("PING_SOCKET_MAX_RETRIES expected as a positive integer");
        let ping_socket_timeout = Duration::from_millis(ping_socket_timeout);
        PingEnv {
            ping_socket_timeout,
            ping_socket_max_retries,
        }
    }
    pub fn get_ping_socket_timeout() -> Duration {
        PingEnv::get_vars().ping_socket_timeout
    }
    pub fn get_ping_socket_max_retries() -> u8 {
        PingEnv::get_vars().ping_socket_max_retries
    }
}
impl Env {
    pub fn get_vars() -> Self {
        let database_settings = DatabaseEnv::get_vars();
        let tcp_comunication_settings = ComunicationEnv::get_vars();
        let websocket_settings = WebsocketEnv::get_vars();
        let channels_settings = ChannelsEnv::get_vars();

        Env {
            database_settings,
            tcp_comunication_settings,
            websocket_settings,
            channels_settings
        }
    }
}
