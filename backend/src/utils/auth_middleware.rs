use crate::utils::{common::toast, jwt_utils::jwt_to_id};

use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    middleware::Next,
    Error, HttpResponse,
};
use jsonwebtoken::errors::ErrorKind;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let auth_header = match req.headers().get(header::AUTHORIZATION) {
        Some(header) => header,
        None => {
            let res = HttpResponse::Unauthorized().json(toast(
                "No 'Authorization' header provided, pleas provide one.",
            ));
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    };

    let auth_header = match auth_header.to_str() {
        Ok(string_header) => String::from(string_header),
        Err(_) => {
            let res = HttpResponse::Unauthorized().json(toast(
                "Authorization header has invalid format.",
            ));
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    };

    let jwt_token = match auth_header.strip_prefix("Bearer ") {
        Some(jwt_token) => String::from(jwt_token),
        None => {
            let res = HttpResponse::Unauthorized()
                .json(toast("Invalid token, expected Bearer token."));
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    };

    if let Err(error) = jwt_to_id(jwt_token) {
        match error.kind() {
            ErrorKind::ExpiredSignature => {
                let res = HttpResponse::Unauthorized()
                    .json(toast("Token signature has expired."));
                return Ok(ServiceResponse::new(req.into_parts().0, res));
            }

            _ => {
                let res = HttpResponse::Unauthorized()
                    .json(toast("You've provided an invalid token."));
                return Ok(ServiceResponse::new(req.into_parts().0, res));
            }
        }
    }

    next.call(req).await
}
