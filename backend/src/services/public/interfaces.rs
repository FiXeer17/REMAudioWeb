use std::net::SocketAddrV4;
use super::schemas::*;
use super::utils::{insert_user, SRC};
use crate::utils::configs::{channels_settings,DatabaseEnv};
use crate::AppState;
use crate::services::public::signin::schemas::ReturnFullUser;
use actix_web::web::Data;
use sqlx::Row;

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
    insert_user(default_admin, default_admin_password,true, pgpool).await?;
    insert_user(default_user, default_user_password, false,pgpool).await?;
    Ok(())
}

pub async fn retrieve_admin_from_username(pgpool: &Data<AppState>, username: &str) -> Result<bool, sqlx::Error> {
    let query_string: &str = "SELECT admin FROM users WHERE username = $1 AND deleted_at IS NULL;";
    let result: sqlx::postgres::PgRow = sqlx::query(query_string)
        .bind(username.to_string())
        .fetch_one(&pgpool.db)
        .await?;
    Ok(result.get("admin"))
}
pub async fn retrieve_admin_from_id(pgpool: &Data<AppState>, id: i32) -> Result<bool, sqlx::Error> {
    let query_string: &str = "SELECT admin FROM users WHERE id = $1 AND deleted_at IS NULL;";
    let result = sqlx::query(query_string)
        .bind(id)
        .fetch_one(&pgpool.db)
        .await?;
    Ok(result.get("admin"))
}

pub async fn retrieve_channels(
    pgpool: &AppState,
    socket_id : i32,
    src: SRC,
) -> Result<Option<Vec<Channel>>, sqlx::Error> {
    let query_string: &str = "SELECT * FROM channels WHERE socket_id=$1 AND src=$2 ORDER BY relative_identifier ASC;";
    let channels: Vec<Channel> = sqlx::query_as::<_, Channel>(query_string)
        .bind(socket_id)
        .bind(src.to_string())
        .fetch_all(&pgpool.db)
        .await?;

    if channels.is_empty() {
        return Ok(None);
    }
    return Ok(Some(channels));
}

pub async fn add_io_channels(pgpool: &AppState, socket_id: i32) -> Result<(),sqlx::Error>{
    let query_string: &str ="INSERT INTO channels (channel_name,visible,src,socket_id,relative_identifier) VALUES ($1,$2,$3,$4,$5);";

    let (i_channels, o_channels, default_visibility, channel_prefix) = (
        channels_settings::get_i_channel_number(),
        channels_settings::get_o_channel_number(),
        channels_settings::get_default_visibility(),
        channels_settings::get_channel_default_prefix(),
    );
    
    for i in 1..i_channels+1{
        sqlx::query(query_string)
        .bind(format!("{}{}",channel_prefix,i))
        .bind(default_visibility)
        .bind(SRC::INPUT.to_string())
        .bind(socket_id)
        .bind(i as i32)
        .fetch_optional(&pgpool.db).await?;
    }
    for i in 1..o_channels+1{
        sqlx::query(query_string)
        .bind(format!("{}{}",channel_prefix,i))
        .bind(default_visibility)
        .bind(SRC::OUTPUT.to_string())
        .bind(socket_id)
        .bind(i as i32)
        .fetch_optional(&pgpool.db).await?;
    }

    Ok(())
}

pub async fn update_channel_visibility(pgpool: &AppState,socket_id: i32,relative_identifier:i32,visibility:bool,src:String)->Result<(),sqlx::Error>{
    let query_string: &str = "UPDATE channels SET visible=$1 WHERE socket_id=$2 AND relative_identifier=$3 AND src=$4;";
    sqlx::query(query_string)
    .bind(visibility)
    .bind(socket_id)
    .bind(relative_identifier)
    .bind(src)
    .fetch_optional(&pgpool.db).await?;

    Ok(())
}

pub async fn retrieve_sockets(pgpool: &AppState)->Result<Vec<Socket>,sqlx::Error>{
    let query_string: &str = "SELECT * FROM sockets";
    let sockets= sqlx::query_as::<_, Socket>(query_string).fetch_all(&pgpool.db).await?;
    Ok(sockets)
}

pub async fn insert_socket_in_db(pgpool: &AppState,socket_name:String,socket:SocketAddrV4)->Result<(),sqlx::Error>{
    let query_string: &str = "INSERT INTO sockets (socket_name,socket,latest) VALUES ($1,$2,$3);";
    sqlx::query(query_string)
    .bind(socket_name)
    .bind(socket.to_string())
    .bind(true)
    .fetch_optional(&pgpool.db).await?;

    Ok(())
}
pub async fn remove_socket_in_db(pgpool: &AppState,socket:SocketAddrV4)->Result<(),sqlx::Error>{
    let query_string: &str = "DELETE FROM sockets WHERE socket = $1";
    sqlx::query(query_string)
    .bind(socket.to_string())
    .fetch_optional(&pgpool.db).await?;
    Ok(())

}

pub async fn update_latest_socket_in_db(pgpool: &AppState,socket:SocketAddrV4)->Result<(),sqlx::Error>{
    let update_other_latest_query: &str = "UPDATE sockets SET latest=false;";
    let update_latest_query:&str = "UPDATE sockets SET latest=true WHERE socket=$1;";
    sqlx::query(update_other_latest_query)
    .bind(socket.to_string())
    .fetch_optional(&pgpool.db).await?;

    sqlx::query(update_latest_query)
    .bind(socket.to_string())
    .fetch_optional(&pgpool.db).await?;

    Ok(())
}

pub async fn retrieve_socketid_from_db(pgpool: &AppState,socket:SocketAddrV4)-> Result<i32,sqlx::Error>{
    let query:&str = "SELECT id FROM sockets WHERE socket = $1;";
    let row = sqlx::query(query)
    .bind(socket.to_string())
    .fetch_one(&pgpool.db).await?;

    Ok(row.get("id"))
}

pub async fn retrieve_socket_from_db(pgpool: &AppState,socket:SocketAddrV4)-> Result<bool,sqlx::Error>{
    let query:&str = "SELECT id FROM sockets WHERE socket = $1;";
    let row = sqlx::query(query)
    .bind(socket.to_string())
    .fetch_optional(&pgpool.db).await?;

    match row {
        Some(_) => return Ok(true),
        None => return Ok(false)
    }
}

pub async fn retrieve_visibility(pgpool: &AppState, socket_id:&i32) -> Result<(Vec<bool>,Vec<bool>),sqlx::Error>{
    let socket_id = socket_id.clone();
    let i_channels = retrieve_channels(pgpool, socket_id, SRC::INPUT).await?;
    let o_channels = retrieve_channels(pgpool,socket_id,SRC::OUTPUT).await?;
    if i_channels.is_none() || o_channels.is_none(){
        return Err(sqlx::Error::RowNotFound);
    }
    let (mut i_visibility, mut o_visibility) :(Vec<bool>,Vec<bool>) = (Vec::new(),Vec::new());

    i_channels.iter().for_each(|chs|{
        chs.iter().for_each(|ch|{
            i_visibility.push(ch.visible);
        });
    });
    o_channels.iter().for_each(|chs|{
        chs.iter().for_each(|ch|{
            o_visibility.push(ch.visible);
        });
    });
    Ok((i_visibility,o_visibility))
}

pub async fn retrieve_labels(pgpool: &AppState, socket_id:&i32) -> Result<(Vec<String>,Vec<String>),sqlx::Error>{
    let socket_id = socket_id.clone();
    let i_channels = retrieve_channels(pgpool, socket_id, SRC::INPUT).await?;
    let o_channels = retrieve_channels(pgpool,socket_id,SRC::OUTPUT).await?;
    if i_channels.is_none() || o_channels.is_none(){
        return Err(sqlx::Error::RowNotFound);
    }
    let (mut i_labels, mut o_labels) :(Vec<String>,Vec<String>) = (Vec::new(),Vec::new());

    i_channels.iter().for_each(|chs|{
        chs.iter().for_each(|ch|{
            i_labels.push(ch.channel_name.clone());
        });
    });
    o_channels.iter().for_each(|chs|{
        chs.iter().for_each(|ch|{
            o_labels.push(ch.channel_name.clone());
        });
    });
    Ok((i_labels,o_labels))
}

pub async fn update_labels_in_db(pgpool: &AppState,socket_id: i32,relative_identifier:i32,label:String,src:String)-> Result<(), sqlx::Error>{
    let query_string: &str = "UPDATE channels SET channel_name=$1 WHERE socket_id=$2 AND relative_identifier=$3 AND src=$4;";
    sqlx::query(query_string)
    .bind(label)
    .bind(socket_id)
    .bind(relative_identifier)
    .bind(src)
    .fetch_optional(&pgpool.db).await?;

    Ok(())
}