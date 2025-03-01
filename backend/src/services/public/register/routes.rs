use crate::services::public::{interfaces::insert_user, register::schemas};
use crate::{
    utils::{
        common::return_json_reason,
        hasher::{argon2_enc, id_to_jwt},
    },
    AppState,
};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde_json::to_string_pretty;
use validator::Validate;

#[post("/register")]
pub async fn register(
    request_body: web::Json<schemas::CreateUser>,
    pgpool: Data<AppState>,
) -> impl Responder {
    if let Err(_) = request_body.validate() {
        return HttpResponse::BadRequest().json(return_json_reason("Format not valid, retry later."));
    }

    let hashed_pswd = match argon2_enc(&request_body.password) {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(return_json_reason(&e.to_string()))
        }
    };
    match insert_user(
        pgpool,
        &request_body.username,
        &request_body.email,
        &hashed_pswd,
    )
    .await
    {
        Ok(new_user) => {
            let jwt_token = match id_to_jwt(new_user.id, request_body.session_type.clone()) {
                Ok(token) => token,
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            };

            return HttpResponse::Ok().content_type("application/json").body(
                match to_string_pretty(&schemas::ReturnCreateUserJWT {
                    id: new_user.id,
                    username: new_user.username,
                    email: new_user.email,
                    jwt_token,
                }) {
                    Ok(pretty) => pretty,
                    Err(_) => {
                        return HttpResponse::InternalServerError().finish();
                    }
                },
            );
        }
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::UnprocessableEntity()
                .json(return_json_reason("e-mail already taken."))
        }
        Err(_) => return HttpResponse::BadRequest().finish(),
    }
}
