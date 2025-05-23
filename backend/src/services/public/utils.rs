use crate::{utils::hasher_utils::argon2_enc, AppState};

use super::{interfaces::{check_username, retrieve_channels, retrieve_presets}, schemas::{Channel, Preset}};


pub async fn insert_user(
    username: String,
    password: String,
    admin: bool,
    pgpool: &AppState,
) -> Result<(), sqlx::Error> {
    let (user, password) = (username, password);
    match check_username(pgpool, &user).await {
        Ok(true) => return Ok(()),
        Ok(false) => (),
        Err(error) => return Err(error),
    };

    let password = argon2_enc(&password).unwrap();
    let query_string = "INSERT INTO users (username,password,admin) VALUES ($1,$2,$3);";
    let query = sqlx::query(query_string)
        .bind(user)
        .bind(password.to_string())
        .bind(admin)
        .fetch_optional(&pgpool.db)
        .await;
    match query {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => (),
        Err(e) => return Err(e),
    };
    Ok(())
}

pub enum SRC {
    INPUT,
    OUTPUT,
}
impl ToString for SRC {
    fn to_string(&self) -> String {
        match self {
            SRC::INPUT => String::from("input"),
            SRC::OUTPUT => String::from("output"),
        }
    }
}

pub async fn retrieve_all_channels(
    pgpool: &AppState,
    socket_id:i32
) -> Result<Option<Vec<Channel>>, sqlx::Error> {
    let mut results: Vec<Channel> = Vec::new();
    let i_channels = retrieve_channels(pgpool, socket_id, SRC::INPUT).await?;
    let o_channels = retrieve_channels(pgpool, socket_id, SRC::OUTPUT).await?;
    if i_channels.is_none() || o_channels.is_none(){
        return Ok(None);
    }
    results.extend(i_channels.unwrap().into_iter());
    results.extend(o_channels.unwrap().into_iter());
    Ok(Some(results))
}

pub async fn retrieve_all_presets(
    pgpool: &AppState,
    socket_id:i32
) -> Result<Option<Vec<Preset>>, sqlx::Error>{
    let presets = retrieve_presets(pgpool, socket_id).await?;
    Ok(presets)
}
