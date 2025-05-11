use super::schemas::*;
use super::utils::{insert_user, SRC};
use crate::configs::{channels_settings, presets_settings, DatabaseEnv};
use crate::services::private::socket::utils::Device;
use crate::services::public::signin::schemas::ReturnFullUser;
use crate::AppState;
use actix_web::web::Data;
use sqlx::Row;
use std::net::SocketAddrV4;

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
    let query_string = "SELECT id,username,admin,password FROM users WHERE username = $1 AND deleted_at IS NULL;";
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
    insert_user(default_admin, default_admin_password, true, pgpool).await?;
    insert_user(default_user, default_user_password, false, pgpool).await?;
    Ok(())
}

pub async fn retrieve_admin_from_username(
    pgpool: &Data<AppState>,
    username: &str,
) -> Result<bool, sqlx::Error> {
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
    socket_id: i32,
    src: SRC,
) -> Result<Option<Vec<Channel>>, sqlx::Error> {
    let query_string: &str =
        "SELECT * FROM channels WHERE socket_id=$1 AND src=$2 ORDER BY relative_identifier ASC;";
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

pub async fn add_io_channels(pgpool: &AppState, socket_id: i32) -> Result<(), sqlx::Error> {
    let query_string: &str ="INSERT INTO channels (channel_name,visible,src,socket_id,relative_identifier) VALUES ($1,$2,$3,$4,$5);";

    let (channels_number, default_visibility, channel_prefix) = (
        channels_settings::get_channels_number(),
        channels_settings::get_default_visibility(),
        channels_settings::get_channel_default_prefix(),
    );

    for i in 1..channels_number + 1 {
        sqlx::query(query_string)
            .bind(format!("{}{}", channel_prefix, i))
            .bind(default_visibility)
            .bind(SRC::INPUT.to_string())
            .bind(socket_id)
            .bind(i as i32)
            .fetch_optional(&pgpool.db)
            .await?;
    }
    for i in 1..channels_number + 1 {
        sqlx::query(query_string)
            .bind(format!("{}{}", channel_prefix, i))
            .bind(default_visibility)
            .bind(SRC::OUTPUT.to_string())
            .bind(socket_id)
            .bind(i as i32)
            .fetch_optional(&pgpool.db)
            .await?;
    }

    Ok(())
}

pub async fn add_presets(
    pgpool: &AppState,
    socket_id: i32,
    device: String,
) -> Result<(), sqlx::Error> {
    let query_string: &str =
        "INSERT INTO presets (label,relative_identifier,socket_id) VALUES ($1,$2,$3);";
    let np: u8;
    let st: u8;
    let prfx: String;
    if device == Device::Audio.to_string() {
        st = 1;
        np = presets_settings::get_audio_presets_number() +1 ;
        prfx = presets_settings::get_audio_preset_label_prefix()
    } else {
        st=0;
        np = presets_settings::get_video_presets_number();
        prfx = presets_settings::get_video_preset_label_prefix()
    }
    for i in st..np {
        sqlx::query(query_string)
            .bind(format!("{}{}",prfx,i))
            .bind(i as i32)
            .bind(socket_id)
            .fetch_optional(&pgpool.db)
            .await?;
    }
    Ok(())
}

pub async fn update_channel_visibility(
    pgpool: &AppState,
    socket_id: i32,
    relative_identifier: i32,
    visibility: bool,
    src: String,
) -> Result<(), sqlx::Error> {
    let query_string: &str =
        "UPDATE channels SET visible=$1 WHERE socket_id=$2 AND relative_identifier=$3 AND src=$4;";
    sqlx::query(query_string)
        .bind(visibility)
        .bind(socket_id)
        .bind(relative_identifier)
        .bind(src)
        .fetch_optional(&pgpool.db)
        .await?;

    Ok(())
}

pub async fn retrieve_sockets(pgpool: &AppState) -> Result<Vec<Socket>, sqlx::Error> {
    let query_string: &str = "SELECT * FROM sockets";
    let sockets = sqlx::query_as::<_, Socket>(query_string)
        .fetch_all(&pgpool.db)
        .await?;
    Ok(sockets)
}

pub async fn insert_socket_in_db(
    pgpool: &AppState,
    socket_name: String,
    socket: SocketAddrV4,
    device: String,
) -> Result<(), sqlx::Error> {
    let mut tx = pgpool.db.begin().await?;
    let query_string: &str =
        "INSERT INTO sockets (socket_name,socket,latest,device) VALUES ($1,$2,$3,$4)
        ON CONFLICT (socket)
        DO UPDATE SET
            socket_name = EXCLUDED.socket_name,
            device = EXCLUDED.device,
            latest = EXCLUDED.latest;";
    sqlx::query(query_string)
        .bind(socket_name)
        .bind(socket.to_string())
        .bind(true)
        .bind(device.clone())
        .execute(&mut *tx)
        .await?;

    let query_string :&str = "UPDATE sockets SET latest=false WHERE socket != $1 AND device = $2;";
    sqlx::query(query_string)
        .bind(socket.to_string())
        .bind(device)
        .execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}
pub async fn remove_socket_in_db(
    pgpool: &AppState,
    socket: SocketAddrV4,
) -> Result<(), sqlx::Error> {
    let query_string: &str = "DELETE FROM sockets WHERE socket = $1";
    sqlx::query(query_string)
        .bind(socket.to_string())
        .fetch_optional(&pgpool.db)
        .await?;
    Ok(())
}


pub async fn retrieve_socketid_from_db(
    pgpool: &AppState,
    socket: SocketAddrV4,
) -> Result<i32, sqlx::Error> {
    let query: &str = "SELECT id FROM sockets WHERE socket = $1;";
    let row = sqlx::query(query)
        .bind(socket.to_string())
        .fetch_one(&pgpool.db)
        .await?;

    Ok(row.get("id"))
}

pub async fn is_socket_in_db(pgpool: &AppState, socket: SocketAddrV4) -> Result<bool, sqlx::Error> {
    let query: &str = "SELECT id FROM sockets WHERE socket = $1;";
    let row = sqlx::query(query)
        .bind(socket.to_string())
        .fetch_optional(&pgpool.db)
        .await?;

    match row {
        Some(_) => return Ok(true),
        None => return Ok(false),
    }
}
pub async fn retrieve_socket_from_db(
    pgpool: &AppState,
    socket: SocketAddrV4,
) -> Result<Socket, sqlx::Error> {
    let query_string = "SELECT * FROM sockets WHERE socket = $1;";

    sqlx::query_as::<_, Socket>(query_string)
        .bind(socket.to_string())
        .fetch_one(&pgpool.db)
        .await
}

pub async fn retrieve_visibility(
    pgpool: &AppState,
    socket_id: &i32,
) -> Result<(Vec<bool>, Vec<bool>), sqlx::Error> {
    let socket_id = socket_id.clone();
    let i_channels = retrieve_channels(pgpool, socket_id, SRC::INPUT).await?;
    let o_channels = retrieve_channels(pgpool, socket_id, SRC::OUTPUT).await?;
    if i_channels.is_none() || o_channels.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }
    let (mut i_visibility, mut o_visibility): (Vec<bool>, Vec<bool>) = (Vec::new(), Vec::new());

    i_channels.iter().for_each(|chs| {
        chs.iter().for_each(|ch| {
            i_visibility.push(ch.visible);
        });
    });
    o_channels.iter().for_each(|chs| {
        chs.iter().for_each(|ch| {
            o_visibility.push(ch.visible);
        });
    });
    Ok((i_visibility, o_visibility))
}

pub async fn retrieve_channel_labels(
    pgpool: &AppState,
    socket_id: &i32,
) -> Result<(Vec<String>, Vec<String>), sqlx::Error> {
    let socket_id = socket_id.clone();
    let i_channels = retrieve_channels(pgpool, socket_id, SRC::INPUT).await?;
    let o_channels = retrieve_channels(pgpool, socket_id, SRC::OUTPUT).await?;
    if i_channels.is_none() || o_channels.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }
    let (mut i_labels, mut o_labels): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());

    i_channels.unwrap().iter().for_each(|ch| {i_labels.push(ch.channel_name.clone());});
    o_channels.unwrap().iter().for_each(|ch| {o_labels.push(ch.channel_name.clone());});

    Ok((i_labels, o_labels))
}
pub async fn retrieve_preset_labels(
    pgpool: &AppState,
    socket_id: &i32,
) -> Result<Vec<String>, sqlx::Error> {
    let socket_id = socket_id.clone();
    let presets = retrieve_presets(pgpool, socket_id).await?;
    if presets.is_none(){return Err(sqlx::Error::RowNotFound);}
    Ok(presets.unwrap().into_iter().map(|p|{p.label}).collect::<Vec<String>>())
    
}

pub async fn update_latest_preset_in_sockets_db(
    pgpool: &AppState,
    socket_id: i32,
    latest_preset:i32
)-> Result<(),sqlx::Error>{
    let query_string : &str = "UPDATE sockets SET latest_preset=$1 WHERE id=$2; ";
    sqlx::query(query_string)
        .bind(latest_preset)
        .bind(socket_id)
        .execute(&pgpool.db).await?;
    Ok(())
}

pub async fn update_channel_labels_in_db(
    pgpool: &AppState,
    socket_id: i32,
    relative_identifier: i32,
    label: String,
    src: String,
) -> Result<(), sqlx::Error> {
    let query_string: &str = "UPDATE channels SET channel_name=$1 WHERE socket_id=$2 AND relative_identifier=$3 AND src=$4;";
    sqlx::query(query_string)
        .bind(label)
        .bind(socket_id)
        .bind(relative_identifier)
        .bind(src)
        .execute(&pgpool.db)
        .await?;

    Ok(())
}

pub async fn update_preset_labels_in_db(
    pgpool: &AppState,
    socket_id: i32,
    relative_identifier: i32,
    label: String,
) -> Result<(), sqlx::Error> {
    let query_string: &str = "UPDATE presets SET label=$1 WHERE socket_id=$2 AND relative_identifier=$3";
    sqlx::query(query_string)
        .bind(label)
        .bind(socket_id)
        .bind(relative_identifier)
        .execute(&pgpool.db)
        .await?;

    Ok(())
}

pub async fn retrieve_presets(
    pgpool: &AppState,
    socket_id: i32,
) -> Result<Option<Vec<Preset>>, sqlx::Error> {
    let query_string: &str =
        "SELECT * FROM presets WHERE socket_id=$1 ORDER BY relative_identifier ASC;";
    let channels: Vec<Preset> = sqlx::query_as::<_, Preset>(query_string)
        .bind(socket_id)
        .fetch_all(&pgpool.db)
        .await?;

    if channels.is_empty() {
        return Ok(None);
    }
    return Ok(Some(channels));
}
