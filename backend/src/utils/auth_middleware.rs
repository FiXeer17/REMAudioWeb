use crate::utils::{hasher::jwt_to_id,common::return_json_reason};

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
            let res = HttpResponse::Unauthorized().json(return_json_reason(
                "No 'Authorization' header provided, pleas provide one.",
            ));
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    };

    let auth_header = match auth_header.to_str() {
        Ok(string_header) => String::from(string_header),
        Err(_) => {
            let res = HttpResponse::Unauthorized().json(return_json_reason(
                "Authorization header has invalid format.",
            ));
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    };

    let jwt_token = match auth_header.strip_prefix("Bearer ") {
        Some(jwt_token) => String::from(jwt_token),
        None => {
            let res = HttpResponse::Unauthorized()
                .json(return_json_reason("Invalid token, expected Bearer token."));
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    };

    if let Err(error) = jwt_to_id(jwt_token) {
        match error.kind() {
            ErrorKind::ExpiredSignature => {
                let res = HttpResponse::Unauthorized()
                    .json(return_json_reason("Token signature has expired."));
                return Ok(ServiceResponse::new(req.into_parts().0, res));
            }

            _ => {
                let res = HttpResponse::Unauthorized()
                    .json(return_json_reason("You've provided an invalid token."));
                return Ok(ServiceResponse::new(req.into_parts().0, res));
            }
        }
    }

    next.call(req).await
}
