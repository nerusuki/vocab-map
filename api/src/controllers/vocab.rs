use crate::services::vocab;
use crate::utils::response;

use actix_web::{
    HttpRequest, HttpResponse, Responder, Scope,
    http::header::{HeaderMap, HeaderName},
    web,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

async fn get(req: HttpRequest) -> impl Responder {
    let headers: &HeaderMap = req.headers();

    let Some(authorization) = headers.get(HeaderName::from_static("authorization")) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let authorization = authorization.to_str().unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let token_str = authorization.replace("Bearer ", "");
    let mut claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    let user_id = claims.remove("sub").unwrap();

    let Ok(words) = vocab::get_user(user_id.parse().unwrap()) else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/vocab").route("", web::get().to(get))
}
