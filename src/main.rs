mod models;
mod error;
// mod config;
mod client;
mod services;
mod handlers;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use handlers::pokemon::get_pokemon;
use std::sync::Arc;
use crate::services::PokemonService;

#[tokio::main]
async fn main() {
    println!("Starting Pokemon API server...");
    println!("Pokemon server running on http://127.0.0.1:3000");

    let pokemon_service = Arc::new(PokemonService::new());

    let app = Router::new()
        .route("/", get(health_check))
        .route("/pokemon/:name_or_id", get(get_pokemon))
        .with_state(pokemon_service);

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