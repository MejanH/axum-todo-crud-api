mod db;
mod handlers;
mod models;
mod routes;

use crate::db::init_db;
use crate::routes::create_routes;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let db = init_db();
    let app = create_routes(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on: http://localhost:5000");
    axum::serve(listener, app).await.unwrap();
}
