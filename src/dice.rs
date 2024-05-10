#![allow(dead_code)]
use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use log::{error, info};
use rand::prelude::*;
use rand::thread_rng as rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Roll(i32);

pub enum DiceKind {
    Faced, //D20, D10, D100
    Fate,  // + - 0
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RollResult {
    FacedRollResult(Vec<Roll>),
    FateRollResult(Vec<Roll>),
}

impl Roll {
    pub fn new() -> Self {
        Roll(0)
    }
    pub fn roll(&self, dice: DiceKind, range: Option<i32>, times: Option<i32>) -> RollResult {
        match dice {
            DiceKind::Fate => self.fate_roll(),
            DiceKind::Faced => self.faced_roll(range.unwrap_or(20), times.unwrap_or(1)),
        }
    }
    fn fate_roll(&self) -> RollResult {
        let mut rng = rand::thread_rng();
        let rolls: Vec<Roll> = (0..4)
            .map(|_| {
                let roll_value = rng.gen_range(-1..=1);
                Roll(roll_value)
            })
            .collect();
        RollResult::FateRollResult(rolls)
    }
    fn faced_roll(&self, range: i32, mut times: i32) -> RollResult {
        if times.is_negative() {
            eprintln!("Error: Cannot roll negative number of times, defaulting to 1");
            times = 1
        };
        let rolls: Vec<Roll> = (1..=times)
            .map(|_| Roll(rng().gen_range(1..=range)))
            .collect();
        RollResult::FacedRollResult(rolls)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Action {
    name: Arc<String>,
    path: Arc<String>,
    method: (String, String),
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Menu {
    actions: Vec<Action>,
}
impl Action {
    fn new(name: &str, path: &str, method: (&str, &str)) -> Self {
        Action {
            name: Arc::new(name.to_string()),
            path: Arc::new(String::from("/dice/") + path),
            method: (String::from(method.0), String::from(method.1)),
        }
    }

    pub(crate) async fn faced_roll(headers: HeaderMap) -> impl IntoResponse {
        // How many times to roll
        let times = headers
            .get("times")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i32>().ok())
            .or(Some(1));

        // Dice roll range, D20, D12...
        let range = headers
            .get("range")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i32>().ok())
            .or(Some(20));

        // Dice rolling
        let roll = Roll::new().roll(DiceKind::Faced, range, times);
        (StatusCode::NOT_FOUND, Json(roll))
    }

    pub(crate) async fn fate_roll(headers: HeaderMap) -> Json<Vec<RollResult>> {
        // parse headers and handle defaults
        let times = headers
            .get("times")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(1);

        // handle rolls
        let roll_vec: Vec<RollResult> = (1..=times)
            .map(|_| {
                info!("Rolling fate dice {} times.", times);
                Roll::new().roll(DiceKind::Fate, None, None)
            })
            .collect();
        Json(roll_vec)
    }
}

// Generates the menu for the root endpoint of the module.
impl Menu {
    fn new() -> Self {
        let actions: Vec<Action> = vec![
            Action::new("Application Menu", "", ("get", "menu_routes")),
            Action::new("Roll for FATE Dices", "fate", ("post", "fate_roll")),
            Action::new(
                "Roll for Faced Dices (D20, D12...)",
                "faced",
                ("post", "faced_roll"),
            ),
        ];
        Menu { actions }
    }
}

/// Returns a JSON containing the Route Options
pub(crate) async fn menu_routes() -> Json<serde_json::Value> {
    let menu = Menu::new();
    Json(json!(menu))
}

// TODO: Implement a way to dynamically generate all routing,
// with their respective handlers and paths.
//
// pub(crate) async fn create_route(action: &[Action]) -> Router {
//     let router = Router::new();
//     let routes = action.iter().map(|action| {
//         let f = match &action.method.0.to_uppercase()[..] {
//             "GET" => router.route(&action.path.as_str(), get(||-> impl fn action.method.1() )),
//             _ => panic!()
//         };
//     }).collect();
//         routes
// }
