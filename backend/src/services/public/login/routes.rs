use crate::services::public::{interfaces::from_email, login::schemas};
use crate::{
    utils::{
        common::return_json_reason,
        hasher::{argon2_verify, id_to_jwt},
    },
    AppState,
};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

#[post("/signin")]
pub async fn signin(
    request_body: web::Json<schemas::SignIn>,
    pgpool: Data<AppState>,
) -> impl Responder {
    if let Err(_) = request_body.validate() {
        return HttpResponse::BadRequest().json(return_json_reason("Format not valid, retry later."));
    }
    match from_email(&pgpool, &request_body.email).await {
        Ok(user) => match argon2_verify(&user.password, &request_body.password) {
            Ok(true) => {
                let token = match id_to_jwt(user.id, request_body.session_type.clone()) {
                    Ok(value) => value,
                    Err(_) => {
                        return HttpResponse::InternalServerError().finish();
                    }
                };
                return HttpResponse::Ok().json(json!({"access_token":token}));
            }
            Ok(false) => {
                return HttpResponse::Unauthorized().json(return_json_reason("Wrong credentials."));
            }
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(_) => {
            return HttpResponse::NotFound().json(return_json_reason("Email not found."));
        }
    }
}
