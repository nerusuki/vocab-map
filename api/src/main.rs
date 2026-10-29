use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use generator::generate_missing;
use std::process::exit;

mod controller;
mod db;
mod generator;
mod models;
mod repository;
mod schema;
mod service;
mod tsne;
mod util;

const HOST: &'static str = "localhost";
const PORT: u16 = 8080;

fn main() {
    let mut args = std::env::args();
    args.next();

    let Some(command) = args.next() else {
        print_usage_and_exit();
    };

    match command.as_str() {
        "api" => api().map_err(|err| err.to_string()),
        "generate" => generate_missing().map_err(|err| err.to_string()),
        _ => print_usage_and_exit(),
    }
    .map_err(|err| {
        println!("Error: {}", err);
        exit(1)
    });
}

fn print_usage_and_exit() -> ! {
    let exe = std::env::args().next().unwrap();
    println!("Usage: {} <command>", exe);
    println!("Available commands: api, generate");
    std::process::exit(1)
}

#[actix_web::main]
async fn api() -> std::io::Result<()> {
    println!("Starting API at {}:{}", HOST, PORT);

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
    .bind((HOST, PORT))?
    .run()
    .await
}
