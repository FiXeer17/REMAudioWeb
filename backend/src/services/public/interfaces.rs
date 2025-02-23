use crate::services::public::login::schemas::ReturnFullUser;
use crate::services::public::register::schemas::ReturnCreateUser;
use crate::AppState;
use actix_web::web::Data;

pub async fn check_email(
    pgpool: &Data<AppState>,
    email: &str,
    table: &str,
) -> Result<bool, sqlx::Error> {
    let query_string = format!(
        "SELECT email FROM {} WHERE email = $1 AND deleted_at IS NULL;",
        table
    );
    let result = sqlx::query(&query_string)
        .bind(email.to_string())
        .fetch_optional(&pgpool.db)
        .await?;
    Ok(result.is_some())
}

pub async fn from_email(
    pgpool: &Data<AppState>,
    email: &str,
    table: &str,
) -> Result<ReturnFullUser, sqlx::Error> {
    let query = format!(
        "SELECT id,username,email,admin,password FROM {} WHERE email = $2 ",
        table
    );
    match sqlx::query_as::<_, ReturnFullUser>(&query)
        .bind(table)
        .bind(email)
        .fetch_optional(&pgpool.db)
        .await?
    {
        Some(user) => Ok(user),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn from_id(
    pgpool: &Data<AppState>,
    id: i32,
    table: &str,
) -> Result<ReturnFullUser, sqlx::Error> {
    let query = format!(
        "SELECT id,username,email,admin,password FROM {} WHERE id = $2 ",
        table
    );
    match sqlx::query_as::<_, ReturnFullUser>(&query)
        .bind(table)
        .bind(id)
        .fetch_optional(&pgpool.db)
        .await?
    {
        Some(user) => Ok(user),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn insert_user(
    pgpool: Data<AppState>,
    username: &str,
    email: &str,
    password: &str,
    table: &str,
) -> Result<ReturnCreateUser, sqlx::Error> {
    match check_email(&pgpool, email, table).await {
        Ok(true) => return Err(sqlx::Error::RowNotFound),
        Ok(false) => (),
        Err(error) => return Err(error),
    };
    let query_string = format!(
        "INSERT INTO {} (username,email,password) VALUES ($1,$2,$3) RETURNING *;",
        table
    );
    match sqlx::query_as::<_, ReturnCreateUser>(&query_string)
        .bind(username.to_string())
        .bind(email.to_string())
        .bind(password.to_string())
        .fetch_one(&pgpool.db)
        .await
    {
        Ok(new_user) => return Ok(new_user),
        Err(e) => Err(e),
    }
}
