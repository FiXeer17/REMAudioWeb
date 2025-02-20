use actix_web::{post, web::{self, Data, Json, ReqData}, HttpResponse, Responder};
use serde::de::value;
use validator::Validate;
use crate::{hasher::{argon2_enc,id_to_jwt,argon2_verify}, interfaces::{insert_user,from_email}, schemas};
use crate::AppState;
use serde_json::{json, to_string_pretty, Value};


fn return_json_reason(reason:&str) -> Value{
    json!({"reason":reason})
}

#[post("/api/register")]
pub async fn register(
    request_body: web::Json<schemas::CreateUser>,
    pgpool : Data<AppState>
) -> impl Responder {
    if let Err(_) = request_body.validate() {
        return HttpResponse::BadRequest().json(return_json_reason("validation error."));
    }

    let hashed_pswd = match argon2_enc(&request_body.password){
        Ok(hash) => hash,
        Err(_) => {return HttpResponse::InternalServerError().json(return_json_reason("hashing password error."))
    }
    };

    match insert_user(pgpool, &request_body.username, &request_body.email, &hashed_pswd).await{
        Ok(new_user)  => { 
            let jwt_token = match id_to_jwt(new_user.id, request_body.session_type.clone()){
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
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::UnprocessableEntity().json(return_json_reason("e-mail already taken."))
        },
        Err(_) => {return HttpResponse::BadRequest().finish()}
    }
}



#[post("/api/signin")]
pub async fn signin(
    request_body: web::Json<schemas::SignIn>,
    pgpool : Data<AppState>
) -> impl Responder{

    if let Err(_) = request_body.validate() {
        return HttpResponse::BadRequest().json(return_json_reason("validation error."));
    }
    match from_email(&pgpool, &request_body.email,"users").await{
        Ok(user) => {
            match argon2_verify(&user.password, &request_body.password){
                Ok(true) => {
                    let token = match id_to_jwt(user.id, request_body.session_type.clone()){
                        Ok(value) => value,
                        Err(_) => {return HttpResponse::InternalServerError().finish();}
                    };
                    return HttpResponse::Ok().json(json!({"jwt_token":token}));
                },
                Ok(false) => {return  HttpResponse::Unauthorized().json(return_json_reason("wrong credentials."));}
                Err(_) => {return HttpResponse::InternalServerError().finish();}
            }
        }
        Err(_) => {return HttpResponse::NotFound().json(return_json_reason("email not found."));}
    }
    
}