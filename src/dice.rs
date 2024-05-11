#![allow(dead_code)]
use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use log::{error, info};
use rand::prelude::*;
use rand::thread_rng as rng;
use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Roll(i32);

#[derive(PartialEq)]
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
            error!("Error: Cannot roll negative number of times, defaulting to 1");
            times = 1
        };
        let rolls: Vec<Roll> = (1..=times)
            .map(|_| Roll(rng().gen_range(1..=range)))
            .collect();
        RollResult::FacedRollResult(rolls)
    }
}
