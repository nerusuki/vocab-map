use crate::services::vocab;
use crate::utils::response;
use crate::utils::token::get_user_id;

use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};

async fn get(req: HttpRequest) -> impl Responder {
    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(words) = vocab::get_user(user_id) else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(words)
}

async fn get_projected(req: HttpRequest) -> impl Responder {
    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(result) = vocab::get_user_projected(user_id) else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(result)
}

pub fn create_scope() -> Scope {
    web::scope("/vocab")
        .route("", web::get().to(get))
        .route("/projected", web::get().to(get_projected))
}
