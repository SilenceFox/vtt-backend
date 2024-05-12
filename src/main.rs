#![allow(dead_code, unused_variables)]
use log::{info, LevelFilter};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use vtt_baxum::errors::Result;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod character;
mod chat;
mod dice;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let app_state = setup_state()?;

    let chat = Router::new()
        .route("/history", get(chat::api::get_chat))
        .route("/join", post(chat::api::join))
        .route("/msg", post(chat::api::send_message))
        .route("/leave", delete(chat::api::leave))
        .route("/roll", post(chat::api::chat_roll));

    let character = Router::new()
        .route("/export", post(character::api::export_sheet))
        .route("/import", post(character::api::import_sheet));

    let dice = Router::new()
        .route("/faced", post(dice::api::faced_roll))
        .route("/fate", post(dice::api::fate_roll));

    let app = Router::new()
        .nest("/chat", chat)
        .nest("/character", character)
        .nest("/dice", dice)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        )
        .with_state(app_state);

    info!("Server started on http://localhost:3030");
    info!("Press Ctrl+C to stop the server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
pub struct AppState {
    chat: Arc<Mutex<chat::Chat>>,
    db: Arc<Mutex<sqlite::Connection>>,
}

fn setup_state() -> Result<Arc<AppState>> {
    let sqlite_conn = Arc::new(Mutex::new(sqlite::open("../data.db")?));
    let chat_state = Arc::new(Mutex::new(chat::Chat::new()));
    let state = AppState {
        chat: chat_state,
        db: sqlite_conn,
    };
    Ok(Arc::new(state))
}
