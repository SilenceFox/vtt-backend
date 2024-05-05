use crate::routable::Routable;
use warp::Filter;
mod character;
pub mod chat;
mod dice;
mod routable;
use log::{error, info, LevelFilter};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let mut chat = chat::Chat::new();
    let user1 = chat::User::new_user("user1".to_string());
    let user2 = chat::User::new_user("user2".to_string());

    macro_rules! msg {
        ($user:expr, $message:expr) => {{
            let user_arc = $user.clone();
            chat.send_message(user_arc, $message.to_string())
        }};
    }

    chat.send_message(user1, "Salve".to_string());
    chat.send_message(user2.clone(), "Eita bicho mensagem KKKKKKKKKK".to_string());
    msg!(user2, "Whats up");

    dbg!(&chat);
    println!("");
    println!("");
    println!("");
    chat.get_history();

    let dice_routes = dice::Menu::menu_routes();
    let rolls = dice::Action::faced_roll().or(dice::Action::fate_roll());
    let character_routes = character::Menu::menu_routes();
    let routes = character_routes.or(dice_routes).or(rolls);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
