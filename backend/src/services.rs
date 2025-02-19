use actix_web::{post, web::{self, Data}, HttpResponse, Responder};
use validator::Validate;
use crate::{hasher::{encrypt,id_to_jwt}, interfaces::insert_user, schemas};
use crate::AppState;
use serde_json::{json, to_string_pretty};

#[post("/api/create_user")]
pub async fn create_user(
    request_body: web::Json<schemas::CreateUser>,
    pgpool : Data<AppState>
) -> impl Responder {
    if let Err(_) = request_body.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let hashed_pswd = match encrypt(&request_body.password){
        Ok(hash) => hash,
        Err(_) => {return HttpResponse::InternalServerError().json(json!({"reason":"hashing password error."}))}
    };

    match insert_user(pgpool, &request_body.username, &request_body.email, &hashed_pswd).await{
        Ok(new_user)  => { 
            let jwt_token = match id_to_jwt(new_user.id){
                Ok(token) => token,
                Err(_) => {return HttpResponse::InternalServerError().finish();}
            };

            return 
            HttpResponse::Ok()
            .content_type("application/json")
            .body(match to_string_pretty(&schemas::ReturnCreateUserJWT{
                id:new_user.id,
                username:new_user.username,
                email:new_user.email,
                jwt_token
            }) {
                Ok(pretty) => pretty,
                Err(_) => {return HttpResponse::InternalServerError().finish();}
            })
        },
        Err(sqlx::Error::RowNotFound) => {return HttpResponse::UnprocessableEntity().json(json!({"reason":"e-mail already taken."}))},
        Err(_) => {return HttpResponse::BadRequest().finish()}
    }
}
