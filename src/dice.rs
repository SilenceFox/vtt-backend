use log::error;
use rand::prelude::*;
use rand::thread_rng as rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use warp::reject::Reject;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

use crate::routable::Routable;

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
                let roll_value = rng.gen_range(1..=3);
                match roll_value {
                    1 => Roll(-1),
                    2 => Roll(0),
                    3 => Roll(1),
                    _ => unreachable!(), // This should never happen
                }
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
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Menu {
    actions: Vec<Action>,
}

impl Action {
    pub(crate) fn new(name: &str, path: &str) -> Self {
        Action {
            name: Arc::new(name.to_string()),
            path: Arc::new(String::from("/dice/") + path),
        }
    }
    pub(crate) fn faced_roll() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let post = warp::path("dice")
            .and(warp::path("faced"))
            .and(warp::post())
            .and(warp::header::optional::<i32>("range"))
            .and(warp::header::optional::<i32>("times"))
            .and_then(|range: Option<i32>, times: Option<i32>| async move {
                #[derive(Debug)]
                struct InvalidRangeError;
                impl Reject for InvalidRangeError {}
                if range <= Some(200) && times <= Some(50) {
                    Ok((range, times))
                } else {
                    error!("Tried to roll with invalid range or times");
                    Err(warp::reject::custom(InvalidRangeError))
                }
            })
            .map(|(range, times)| {
                let roll = Roll::new().roll(DiceKind::Faced, range, times);
                warp::reply::json(&roll)
            });
        post
    }
}

impl Menu {
    pub(crate) fn new() -> Self {
        let actions: Vec<Action> = vec![
            Action::new("Roll for FATE Dices", "fate"),
            Action::new("Roll for Faced Dices (D20, D12...)", "faced"),
        ];
        Menu { actions }
    }
}

impl Routable for Menu {
    fn menu_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let menu = json!(Self::new());
        let route = warp::get()
            .and(warp::path("dice"))
            .map(move || warp::reply::json(&menu));
        route
    }
}
