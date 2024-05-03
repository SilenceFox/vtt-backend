use std::sync::Arc;

use rand::prelude::*;
use rand::thread_rng as rng;
use serde::{Deserialize, Serialize};

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
        let int_range = match range {
            Some(range) => range,
            None => 1,
        };
        let roll_times = match times {
            Some(x) => x,
            None => 1,
        };
        match dice {
            DiceKind::Fate => self.fate_roll(),
            DiceKind::Faced => self.faced_roll(int_range, roll_times),
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
}

impl Menu {
    pub(crate) fn new() -> Self {
        let actions: Vec<Action> = vec![Action::new("Fate", "fate"), Action::new("Faced", "faced")];
        Menu { actions }
    }
}
