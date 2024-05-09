#![allow(non_snake_case)]
mod logic;
mod models;

use axum::{routing::post, Json, Router};
use models::{board_action::BoardAction, player_action::PlayerAction};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Start Rust player");

    let app = Router::new().route("/", post(index));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(Json(payload): Json<BoardAction>) -> Json<PlayerAction> {
    Json(logic::strategy::decide(payload))
}
