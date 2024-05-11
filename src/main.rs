#![allow(dead_code, unused_variables)]
use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
mod character;
mod chat;
mod dice;
use log::{info, LevelFilter};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let chat_state = Arc::new(Mutex::new(chat::Chat::new()));

    let app = Router::new()
        .route("/dice/faced", post(dice::api::faced_roll))
        .route("/dice/fate", post(dice::api::fate_roll))
        .route("/chat/history", get(chat::api::get_chat))
        .route("/chat/join", post(chat::api::join))
        .route("/chat/msg", post(chat::api::send_message))
        .route("/chat/leave", post(chat::api::leave))
        .route("/chat/roll", post(chat::api::chat_roll))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .with_state(chat_state);

    info!("Server started on http://localhost:3030");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
