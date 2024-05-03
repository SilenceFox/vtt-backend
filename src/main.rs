use std::{rc::Rc, sync::Arc};

use serde_json::json;
use warp::Filter;

#[tokio::main]
async fn main() {
    let webhook = warp::post()
        .and(warp::path("webhook"))
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            println!("You have sent: {:#?}", body);
            warp::log("Received request");
            warp::reply::json(&body)
        });

    let actions = vec![
        character::Action::new("Create a character", "character/form"),
        character::Action::new("Submit a Character", "character/submit"),
    ];

    let menu = Arc::new(character::Menu::new(actions));
    let character_menu_route = warp::get()
        .and(warp::path("character"))
        .map(move || warp::reply::json(&*menu.clone()));

    let routes = character_menu_route.or(webhook);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod character {
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    #[derive(Serialize, Deserialize, Debug)]
    pub(crate) struct Action {
        name: Arc<String>,
        path: Arc<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub(crate) struct Menu {
        actions: Vec<Action>,
    }

    impl Action {
        pub(crate) fn new(name: &str, path: &str) -> Self {
            Action {
                name: Arc::new(name.to_string()),
                path: Arc::new(path.to_string()),
            }
        }
    }

    impl Menu {
        pub(crate) fn new(actions: Vec<Action>) -> Self {
            Menu { actions }
        }
    }
}
