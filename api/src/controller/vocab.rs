use crate::util::response;
use crate::util::token::get_user_id;
use crate::{db::DbPool, service};

use actix_web::{
    HttpRequest, HttpResponse, Responder, Scope,
    web::{self, ThinData},
};
use serde::Deserialize;

async fn get(ThinData(pool): ThinData<DbPool>, req: HttpRequest) -> impl Responder {
    let vocab_service = service::Vocab::new(pool);

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(words) = vocab_service.get_user(user_id).await else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(words)
}

async fn get_projected(ThinData(pool): ThinData<DbPool>, req: HttpRequest) -> impl Responder {
    let vocab_service = service::Vocab::new(pool);

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(result) = vocab_service.get_user_projected(user_id).await else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(result)
}

async fn add(ThinData(pool): ThinData<DbPool>, req: HttpRequest) -> impl Responder {
    let vocab_service = service::Vocab::new(pool);
    let word: String = req.match_info().load().unwrap();

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(result) = vocab_service.add_user(&word, user_id).await else {
        return HttpResponse::InternalServerError().json(response::message("Could not add word"));
    };

    HttpResponse::Ok().json(response::message(result))
}

#[derive(Deserialize)]
struct Words {
    words: Vec<String>,
}

async fn add_from_words(
    ThinData(pool): ThinData<DbPool>,
    params: web::Json<Words>,
    req: HttpRequest,
) -> impl Responder {
    let vocab_service = service::Vocab::new(pool);
    let words = params.into_inner().words;

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let result = match vocab_service.add_user_from_words(words, user_id).await {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(response::message(&format!("Added word: {}", result)))
}

async fn delete_words(
    ThinData(pool): ThinData<DbPool>,
    params: web::Json<Words>,
    req: HttpRequest,
) -> impl Responder {
    let vocab_service = service::Vocab::new(pool);
    let words = params.into_inner().words;

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let result = match vocab_service.delete_user_words(words, user_id).await {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(response::message(result))
}

async fn search(ThinData(pool): ThinData<DbPool>, req: HttpRequest) -> impl Responder {
    let vocab_service = service::Vocab::new(pool);
    let word: String = req.match_info().load().unwrap();

    let Ok(words) = vocab_service.search(&word).await else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/vocab")
        .route("", web::get().to(get))
        .route("/projected", web::get().to(get_projected))
        .route("/add/{word}", web::put().to(add))
        .route("/add", web::post().to(add_from_words))
        .route("/search/{word}", web::get().to(search))
        .route("/delete", web::post().to(delete_words))
}
