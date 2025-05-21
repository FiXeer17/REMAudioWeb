use std::fs;
use std::time::Duration;

use dotenv::{dotenv, from_filename};
use serde::{Deserialize, Deserializer, Serialize};

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

#[allow(dead_code, unused_variables)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    settings_path: String,
}

#[allow(dead_code, unused_variables)]
#[derive(Serialize, Deserialize, Clone)]
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
#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct channels_settings {
    channel_label_prefix: String,
    channels_number: u8,
    default_visibility: bool,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct tcp_comunication_settings {
    #[serde(deserialize_with = "u64_to_millis")]
    command_delay: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    reconnect_delay: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    read_timeout: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    preset_read_timeout: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    connection_timeout: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    inactivity_timeout: Duration,
    max_connection_retries: u8,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct websocket_settings {
    #[serde(deserialize_with = "u64_to_millis")]
    heartbeat_interval: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    client_timeout: Duration,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct ping_socket_settings {
    #[serde(deserialize_with = "u64_to_millis")]
    ping_socket_timeout: Duration,
    #[serde(deserialize_with = "u64_to_millis")]
    inactive_sockets_polling_interval: Duration,
    ping_socket_max_retries: u8,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct presets_settings {
    audio_preset_label_prefix: String,
    video_preset_label_prefix: String,
    audio_presets_number: u8,
    video_presets_number: u8,
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct streaming_settings {
    frame_rate: u32,
    transport_protocol: String,
    streaming_path: String,
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct GeneralSettings {
    pub channels_settings: channels_settings,
    pub tcp_comunication_settings: tcp_comunication_settings,
    pub websocket_settings: websocket_settings,
    pub ping_socket_settings: ping_socket_settings,
    pub presets_settings: presets_settings,
    pub streaming_settings: streaming_settings,
}

#[allow(dead_code, unused_variables)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Env {
    pub settings: Settings,
    pub database_settings: DatabaseEnv,
    pub general_settings: GeneralSettings,
}
impl Settings {
    pub fn get_vars() -> Self {
        from_filename(".env.local").ok();
        dotenv().ok();
        let settings_path =
            std::env::var("SETTINGS_PATH").expect("failed to retrieve SETTINGS_PATH");
        Settings { settings_path }
    }
    pub fn get_settings_path() -> String {
        Settings::get_vars().settings_path
    }
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

impl GeneralSettings {
    pub fn get_vars() -> Self {
        let path = Settings::get_settings_path();
        let json_datas = fs::read_to_string(path).expect("failed to read to string from path");
        serde_json::from_str::<GeneralSettings>(&json_datas).expect("failed to convert settings")
    }
}

impl channels_settings {
    pub fn get_channels_number() -> u8 {
        GeneralSettings::get_vars()
            .channels_settings
            .channels_number
    }
    pub fn get_default_visibility() -> bool {
        GeneralSettings::get_vars()
            .channels_settings
            .default_visibility
    }
    pub fn get_channel_default_prefix() -> String {
        GeneralSettings::get_vars()
            .channels_settings
            .channel_label_prefix
    }
}
impl ping_socket_settings {
    pub fn get_ping_socket_max_retries() -> u8 {
        GeneralSettings::get_vars()
            .ping_socket_settings
            .ping_socket_max_retries
    }
    pub fn get_ping_socket_timeout() -> Duration {
        GeneralSettings::get_vars()
            .ping_socket_settings
            .ping_socket_timeout
    }
    pub fn get_inactive_sockets_polling_interval() -> Duration {
        GeneralSettings::get_vars()
            .ping_socket_settings
            .inactive_sockets_polling_interval
    }
}
impl websocket_settings {
    pub fn get_heartbeat_interval() -> Duration {
        GeneralSettings::get_vars()
            .websocket_settings
            .heartbeat_interval
    }
    pub fn get_client_timeout() -> Duration {
        GeneralSettings::get_vars()
            .websocket_settings
            .client_timeout
    }
}
impl tcp_comunication_settings {
    pub fn get_read_timeout() -> Duration {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .read_timeout
    }
    pub fn get_preset_read_timeout() -> Duration {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .preset_read_timeout
    }
    pub fn get_command_delay() -> Duration {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .command_delay
    }
    pub fn get_inactivity_timeout() -> Duration {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .inactivity_timeout
    }
    pub fn get_connection_timeout() -> Duration {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .connection_timeout
    }
    pub fn get_reconnect_delay() -> Duration {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .reconnect_delay
    }
    pub fn get_max_connection_retries() -> u8 {
        GeneralSettings::get_vars()
            .tcp_comunication_settings
            .max_connection_retries
    }
}
impl presets_settings {
    pub fn get_audio_preset_label_prefix() -> String {
        GeneralSettings::get_vars()
            .presets_settings
            .audio_preset_label_prefix
    }
    pub fn get_video_preset_label_prefix() -> String {
        GeneralSettings::get_vars()
            .presets_settings
            .video_preset_label_prefix
    }
    pub fn get_audio_presets_number() -> u8 {
        GeneralSettings::get_vars()
            .presets_settings
            .audio_presets_number
    }
    pub fn get_video_presets_number() -> u8 {
        GeneralSettings::get_vars()
            .presets_settings
            .video_presets_number
    }
}

impl streaming_settings {
    pub fn get_frame_rate() -> u32 {
        GeneralSettings::get_vars().streaming_settings.frame_rate
    }
    pub fn get_transport_protocol() -> String {
        GeneralSettings::get_vars()
            .streaming_settings
            .transport_protocol
    }
    pub fn get_streaming_path() -> String {
        GeneralSettings::get_vars()
            .streaming_settings
            .streaming_path
    }
}

impl Env {
    pub fn get_vars() -> Self {
        let database_settings = DatabaseEnv::get_vars();
        let general_settings = GeneralSettings::get_vars();
        let settings = Settings::get_vars();

        Env {
            database_settings,
            general_settings,
            settings,
        }
    }
}

pub fn u64_to_millis<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Duration::from_millis(u64::deserialize(d)?))
}
