use super::utils::{insert_user, Channel, SRC};
use crate::utils::configs::ChannelsEnv;
use crate::AppState;
use crate::{services::public::signin::schemas::ReturnFullUser, utils::configs::DatabaseEnv};
use actix_web::web::Data;

pub async fn check_username(pgpool: &AppState, username: &str) -> Result<bool, sqlx::Error> {
    let query_string: &str =
        "SELECT username FROM users WHERE username = $1 AND deleted_at IS NULL;";
    let result = sqlx::query(query_string)
        .bind(username.to_string())
        .fetch_optional(&pgpool.db)
        .await?;
    Ok(result.is_some())
}

pub async fn from_username(
    pgpool: &Data<AppState>,
    username: &str,
) -> Result<ReturnFullUser, sqlx::Error> {
    let query_string = "SELECT id,username,admin,password FROM users WHERE username = $1 ";
    match sqlx::query_as::<_, ReturnFullUser>(query_string)
        .bind(username)
        .fetch_optional(&pgpool.db)
        .await?
    {
        Some(user) => Ok(user),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn from_id(pgpool: &Data<AppState>, id: i32) -> Result<ReturnFullUser, sqlx::Error> {
    let query = "SELECT id,username,email,admin,password FROM users WHERE id = $1 ";

    match sqlx::query_as::<_, ReturnFullUser>(query)
        .bind(id)
        .fetch_optional(&pgpool.db)
        .await?
    {
        Some(user) => Ok(user),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn insert_default_user(pgpool: &AppState) -> Result<(), sqlx::Error> {
    let default_admin = DatabaseEnv::get_default_admin();
    let default_admin_password = DatabaseEnv::get_default_admin_password();
    let default_user = DatabaseEnv::get_default_user();
    let default_user_password = DatabaseEnv::get_default_user_password();
    insert_user(default_admin, default_admin_password, pgpool).await?;
    insert_user(default_user, default_user_password, pgpool).await?;
    Ok(())
}

pub async fn retrieve_admin(pgpool: &Data<AppState>, username: &str) -> Result<bool, sqlx::Error> {
    let query_string: &str = "SELECT admin FROM users WHERE username = $1 AND deleted_at IS NULL;";
    let result = sqlx::query(query_string)
        .bind(username.to_string())
        .fetch_optional(&pgpool.db)
        .await?;
    Ok(result.is_some())
}

pub async fn retrieve_channels(
    pgpool: &AppState,
    user_id: i32,
    src: SRC,
) -> Result<Option<Vec<Channel>>, sqlx::Error> {
    let query_string: &str = "SELECT * FROM channels WHERE user_id = $1 AND src=$2;";
    let channels: Vec<Channel> = sqlx::query_as::<_, Channel>(query_string)
        .bind(user_id)
        .bind(src.to_string())
        .fetch_all(&pgpool.db)
        .await?;

    if channels.is_empty() {
        return Ok(None);
    }
    return Ok(Some(channels));
}

pub async fn add_io_channels(pgpool: &AppState, user_id: i32) -> Result<(),sqlx::Error>{
    let query_string: &str ="INSERT INTO channels (channel_name,visible,src,user_id) VALUES ($1,$2,$3,$4);";

    let (i_channels, o_channels, default_visibility, channel_prefix) = (
        ChannelsEnv::get_i_channel_number(),
        ChannelsEnv::get_o_channel_number(),
        ChannelsEnv::get_default_visibility(),
        ChannelsEnv::get_channel_default_prefix(),
    );
    
    for i in 1..i_channels{
        sqlx::query(query_string)
        .bind(format!("{}{}",channel_prefix,i))
        .bind(default_visibility)
        .bind(SRC::INPUT.to_string())
        .bind(user_id).fetch_optional(&pgpool.db).await?;
    }
    for i in 1..o_channels{
        sqlx::query(query_string)
        .bind(format!("{}{}",channel_prefix,i))
        .bind(default_visibility)
        .bind(SRC::OUTPUT.to_string())
        .bind(user_id).fetch_optional(&pgpool.db).await?;
    }

    Ok(())
}
