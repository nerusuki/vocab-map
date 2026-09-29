use crate::util::response;
use crate::{db::DbPool, service};

use actix_web::{
    HttpResponse, Responder, Scope,
    web::{self, ThinData},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthParams {
    username: String,
    password: String,
}

async fn auth(ThinData(pool): ThinData<DbPool>, params: web::Json<AuthParams>) -> impl Responder {
    let user_service = service::User::new(pool);

    let token = match user_service.auth(&params.username, &params.password).await {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(token)
}

pub fn create_scope() -> Scope {
    web::scope("/auth").route("", web::post().to(auth))
}
