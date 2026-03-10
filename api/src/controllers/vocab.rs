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

pub fn create_scope() -> Scope {
    web::scope("/vocab").route("", web::get().to(get))
}
