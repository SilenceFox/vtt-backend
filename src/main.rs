use crate::routable::Routable;
use warp::Filter;
mod character;
mod dice;
mod routable;
use log::{error, info, LevelFilter};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let dice_routes = dice::Menu::menu_routes();
    let faced_roll = dice::Action::faced_roll();
    let character_routes = character::Menu::menu_routes();
    let routes = character_routes.or(dice_routes).or(faced_roll);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
