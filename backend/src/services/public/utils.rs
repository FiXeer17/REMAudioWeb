use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{utils::hasher::argon2_enc, AppState};

use super::interfaces::{check_username, retrieve_channels};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Channel {
    pub id: i32,
    pub channel_name: String,
    pub visible: bool,
    pub user_id: i32,
}

pub async fn insert_user(
    username: String,
    password: String,
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
        .bind(true)
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
    user_id: i32,
    pgpool: &AppState,
) -> Result<Option<Vec<Channel>>, sqlx::Error> {
    let mut results: Vec<Channel> = Vec::new();
    let i_channels = retrieve_channels(pgpool, user_id, SRC::INPUT).await?;
    let o_channels = retrieve_channels(pgpool, user_id, SRC::OUTPUT).await?;
    if i_channels.is_none() || o_channels.is_none(){
        return Ok(None);
    }
    results.extend(i_channels.unwrap().into_iter());
    results.extend(o_channels.unwrap().into_iter());
    Ok(Some(results))
}
