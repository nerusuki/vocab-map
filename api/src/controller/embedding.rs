use crate::db::{DbPool, Language};
use crate::service;
use crate::util::response;

use actix_web::web::ThinData;
use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};
use serde::Deserialize;

async fn predict(ThinData(pool): ThinData<DbPool>, req: HttpRequest) -> impl Responder {
    let embedding_service = service::Embedding::new(pool);
    let word: String = req.match_info().load().unwrap();

    #[derive(Deserialize)]
    struct Params {
        count: Option<i64>,
        vocab: Option<String>,
        lang: Option<Language>,
    }

    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    let count = params.count.unwrap_or_else(|| 30);
    let vocab_only = params.vocab.is_some();
    let lang = params.into_inner().lang;

    let words = match embedding_service
        .predict_from_word(&word, count, vocab_only, lang)
        .await
    {
        Ok(words) => words,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/embedding").route("/predict/{word}", web::get().to(predict))
}
