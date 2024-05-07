#![allow(dead_code)]
use axum::Json;
use log::error;
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

#[derive(Debug)]
enum ErrorReply {
    InvalidRange(Arc<str>),
    // NotEnoughArguments,
    // TooManyArguments,
    // InvalidDiceKind,
    // ArgumentNotNumber,
    // CantParse,
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
    // pub(crate) fn faced_roll() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    //     let post = warp::path("dice")
    //         .and(warp::path("faced"))
    //         .and(warp::path::end())
    //         .and(warp::post())
    //         .and(warp::header::optional::<i32>("range"))
    //         .and(warp::header::optional::<i32>("times"))
    //         .and_then(|range: Option<i32>, times: Option<i32>| async move {
    //             if range <= Some(200) && times <= Some(50) {
    //                 Ok((range, times))
    //             } else {
    //                 error!("Tried to roll with invalid range or times");
    //                 Err(warp::reject::custom(ErrorReply::InvalidRange(
    //                     "Please provide appropriate headers,\
    //                     `range` cant exceed 200,\
    //                         `times` cant exceed 50"
    //                         .into(),
    //                 )))
    //             }
    //         })
    //         .map(|(range, times)| {
    //             let roll = Roll::new().roll(DiceKind::Faced, range, times);
    //             warp::reply::json(&roll)
    //         });
    //     post
    // }
    // pub(crate) fn fate_roll() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    //     let post = warp::path("dice")
    //         .and(warp::path("fate"))
    //         .and(warp::path::end())
    //         .and(warp::post())
    //         .and(warp::header::optional::<i32>("times"))
    //         .and_then(|times: Option<i32>| async move {
    //             if times <= Some(4) {
    //                 Ok(times)
    //             } else {
    //                 error!("Does not support rolling more than 4 times");
    //                 Err(warp::reject::custom(ErrorReply::InvalidRange(
    //                     "Please provide appropriate headers,\
    //                         `times` cant exceed 4"
    //                         .into(),
    //                 )))
    //             }
    //         })
    //         .map(|times| {
    //             let roll = Roll::new().roll(DiceKind::Fate, None, times);
    //             warp::reply::json(&roll)
    //         });
    //     post
    // }
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

pub(crate) async fn menu_routes() -> Json<serde_json::Value> {
    let menu = Menu::new();
    Json(json!(menu))

    // Retornar um JSON com as rotas do /dice
}
