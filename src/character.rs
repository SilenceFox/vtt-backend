use crate::routable::Routable;
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::{reject::Rejection, reply::Reply, Filter};
use std::sync::Arc;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Action {
    name: Arc<String>,
    path: Arc<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Menu {
    actions: Vec<Action>,
}

impl Action {
    pub(crate) fn new(name: &str, path: &str) -> Self {
        Action {
            name: Arc::new(name.to_string()),
            path: Arc::new(String::from("/character/") + path),
        }
    }
}

impl Menu {
    pub(crate) fn new() -> Self {
        let actions = vec![
            Action::new("Create a character from a template", "new-template"),
            Action::new("Submit a Character", "submit"),                      
            Action::new("Updates the Character Sheet", "update"),
        ];
        Menu { actions }
    }
}

impl Routable for Menu {
    fn menu_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
         let menu = json!(Self::new());
         let route = warp::get()
             .and(warp::path("character"))
             .map(move || warp::reply::json(&menu));
         route
    }
}
