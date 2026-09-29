use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
mod controller;
mod db;
mod models;
mod repository;
mod schema;
mod service;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_connection_pool();
    let app_data = web::ThinData(pool);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(app_data.clone())
            .service(controller::auth::create_scope())
            .service(controller::embedding::create_scope())
            .service(controller::vocab::create_scope())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
