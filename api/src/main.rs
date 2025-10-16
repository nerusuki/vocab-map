use actix_cors::Cors;
use actix_web::{App, HttpServer};
mod controllers;
mod db;
mod models;
mod schema;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(controllers::auth::create_scope())
            .service(controllers::embeddings::create_scope())
            .service(controllers::vocab::create_scope())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
