use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::graph::Graph;
mod controllers;
mod db;
mod graph;
mod models;
mod schema;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut graph = Graph::new();

    graph.add_node("aaaaa", vec![]);
    graph.add_node("bbbbb", vec![]);
    graph.add_node("ccccc", vec![]);
    graph.add_node("abc", vec![1, 2]);
    graph.add_node("asdfgh", vec![]);

    graph.add_edge(3, 5);

    println!("{:#?}", graph);

    graph.delete_node(3);

    println!("{:#?}", graph);

    graph.delete_node(1);

    println!("{:#?}", graph);

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
