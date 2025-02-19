use actix_web::web::Data;
use crate::{AppState, schemas::{ReturnFullUser,ReturnCreateUser}};



pub async fn check_email(pgpool: &Data<AppState>, email: &str) -> Result<bool,sqlx::Error>{
    let result = sqlx::query("SELECT email FROM users WHERE email = $1 AND deleted_at IS NULL;")
    .bind(email.to_string())
    .fetch_optional(&pgpool.db)
    .await?;
    Ok(result.is_some())
}

pub async fn from_id(pgpool: &Data<AppState>, id: i32, table: &str) -> Result<ReturnFullUser,sqlx::Error>{
    match sqlx::query_as::<_,ReturnFullUser>("SELECT id,username,email,admin FROM $1 WHERE id = $2 ")
    .bind(table)
    .bind(id)
    .fetch_optional(&pgpool.db)
    .await? {
        Some(user) => Ok(user),
        None => Err(sqlx::Error::RowNotFound)
    }   
    
}

pub async fn insert_user(pgpool : Data<AppState>, username: &str, email: &str, password : &str)-> Result<ReturnCreateUser,sqlx::Error>{

    match check_email(&pgpool, email).await{
        Ok(true) => {return Err(sqlx::Error::RowNotFound)},
        Ok(false) => (),
        Err(error) => {return Err(error)}
    };

    match sqlx::query_as::<_,ReturnCreateUser>("INSERT INTO users (username,email,password) VALUES ($1,$2,$3) RETURNING *;")
    .bind(username.to_string())
    .bind(email.to_string())
    .bind(password.to_string())
    .fetch_one(&pgpool.db)
    .await{
        Ok(new_user) => {return Ok(new_user)},
        Err(e) => {Err(e)}
    }
}