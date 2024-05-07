#![allow(dead_code, unused_variables)]
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use chat::{handle_user_join, handle_user_leave, handle_user_send_message};
mod character;
pub mod chat;
mod dice;
use log::{error, info, LevelFilter};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let chat_state = Arc::new(Mutex::new(chat::Chat::new()));
    handle_user_join(String::from("Joao"), &chat_state);
    let get_my_user = |state: &Arc<Mutex<chat::Chat>>| -> Arc<chat::User> {
        let chat = state.lock().unwrap();
        let my_user = chat.get_your_user("Joao").unwrap();
        my_user.clone()
    };
    handle_user_send_message(&get_my_user(&chat_state), "Yoooo".to_string(), &chat_state);
    handle_user_send_message(
        &get_my_user(&chat_state),
        "What is love".to_string(),
        &chat_state,
    );
    handle_user_leave(&get_my_user(&chat_state), &chat_state);

    let app = Router::new().route("/dice", get(dice::menu_routes));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let dice_routes = dice::Menu::menu_routes();
    // let rolls = dice::Action::faced_roll().or(dice::Action::fate_roll());
    // let character_routes = character::Menu::menu_routes();
    // let routes = character_routes.or(dice_routes).or(rolls);
    //
    // warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    //
}
