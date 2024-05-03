use std::sync::Arc;

use warp::Filter;
mod character;
mod dice;

#[tokio::main]
async fn main() {
    let rolls = dice::Roll::new();
    dbg!(rolls.roll(dice::DiceKind::Fate, None, None));
    dbg!(rolls.roll(dice::DiceKind::Faced, Some(2), None));

    let char_menu = Arc::new(character::Menu::new());
    let fate_menu = Arc::new(dice::Menu::new());

    let character_menu_route = warp::get()
        .and(warp::path("character"))
        .map(move || warp::reply::json(&*char_menu.clone()));

    let fate_menu_route = warp::get()
        .and(warp::path("dice"))
        .map(move || warp::reply::json(&fate_menu.clone()));

    let routes = character_menu_route.or(fate_menu_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
