use crate::utils::hasher::argon2_enc;
use crate::{utils::configs::Env,services::public::signin::schemas::ReturnFullUser};
use crate::AppState;
use actix_web::web::Data;

pub async fn check_username(pgpool: &AppState, username: &str) -> Result<bool, sqlx::Error> {
    let query_string: &str = "SELECT username FROM users WHERE username = $1 AND deleted_at IS NULL;";
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

pub async fn insert_default_user(
    pgpool: &AppState) -> Result<(), sqlx::Error> {
    let (username,password) = (Env::get_default_admin(),Env::get_default_admin_password());
    match check_username(pgpool, username.as_str()).await {
            Ok(true) => {return Ok(())},
            Ok(false) => (),
            Err(error) => return Err(error),
        };
    
    let password = argon2_enc(&password).unwrap();
    let query_string = "INSERT INTO users (username,password,admin) VALUES ($1,$2,$3);";
    match sqlx::query(query_string)
        .bind(username.to_string())
        .bind(password.to_string())
        .bind(true)
        .fetch_one(&pgpool.db)
        .await{
            Ok(_) => Ok(()),
            Err(error) => {match error{
                sqlx::Error::RowNotFound => {return Ok(())},
                _ => {return Err(error)}
            }} 
        }
    
}

pub async fn retrieve_admin(pgpool: &Data<AppState>, username: &str) -> Result<bool, sqlx::Error> {
    let query_string: &str = "SELECT admin FROM users WHERE username = $1 AND deleted_at IS NULL;";
    let result = sqlx::query(query_string)
        .bind(username.to_string())
        .fetch_optional(&pgpool.db)
        .await?;
    Ok(result.is_some())
}
