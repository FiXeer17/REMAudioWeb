use crate::services::public::signin::schemas::SignInReturn;
use crate::services::public::{interfaces::from_username, signin::schemas};
use crate::{
    utils::{common::return_json_reason, hasher::argon2_verify, jwt_utils::id_to_jwt},
    AppState,
};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use validator::Validate;

#[post("/signin")]
pub async fn signin(
    request_body: web::Json<schemas::SignIn>,
    pgpool: Data<AppState>,
) -> impl Responder {
    if let Err(_) = request_body.validate() {
        return HttpResponse::BadRequest().json(return_json_reason("user format not valid."));
    }
    let username = &request_body.username;
    match from_username(&pgpool, &username.clone()).await {
        Ok(user) => match argon2_verify(&user.password, &request_body.password) {
            Ok(true) => {
                let token = match id_to_jwt(user.id, request_body.session_type.clone()) {
                    Ok(value) => value,
                    Err(_) => {
                        return HttpResponse::InternalServerError().finish();
                    }
                };
                let admin = user.admin;
                let to_return = SignInReturn{access_token:token,admin};
                return HttpResponse::Ok().json(to_return);
            }
            Ok(false) => {
                return HttpResponse::Unauthorized().json(return_json_reason("Wrong credentials."));
            }
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(_) => {
            return HttpResponse::NotFound().json(return_json_reason("Wrong credentials."));
        }
    }
}
