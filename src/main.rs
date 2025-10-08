mod models;
mod error;
// mod config;
mod client;
mod services;
// mod handlers;

use axum::{routing::get, Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    println!("Starting Pokemon API server...");

    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "Pokemon API is running!"
}