use crate::db::{DbPool, Language};
use crate::service;
use crate::util::response;

use actix_web::web::ThinData;
use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};
use serde::{Deserialize, Serialize};

async fn predict(ThinData(pool): ThinData<DbPool>, req: HttpRequest) -> impl Responder {
    let embedding_service = service::Embedding::new(pool);
    let word: String = req.match_info().load().unwrap();

    #[derive(Deserialize)]
    struct Params {
        count: Option<i64>,
        lang: Option<Language>,
    }

    let words = word
        .split(',')
        .map(|word| word.to_string())
        .collect::<Vec<_>>()
        .to_vec();

    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    let count = params.count.unwrap_or_else(|| 30);
    let lang = params.into_inner().lang;

    let words = match embedding_service.predict(words, count, None, lang).await {
        Ok(words) => words,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    #[derive(Serialize)]
    struct WordDistance {
        word: String,
        distance: f64,
    }

    let words = words
        .into_iter()
        .map(|(word, distance)| WordDistance { word, distance })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/embedding").route("/predict/{word}", web::get().to(predict))
}
