use std::{fmt::Display, str::FromStr};

pub mod briansbrain;
pub mod gameoflife;
pub mod langtonsant;

pub const PROPORTION: f64 = 0.9;

#[derive(Debug)]
pub enum Worlds {
    GameOfLife,
    BriansBrain,
    LangtonsAnt,
}

impl Display for Worlds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Worlds {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gof" | "Game of Life" | "GameOfLife" => Ok(Self::GameOfLife),
            "bb" | "Brian's Brian " | "BriansBrian" => Ok(Self::BriansBrain),
            "la" | "Langton's Ant" | "LangtonsAnt" => Ok(Self::LangtonsAnt),
            _ => Err(String::from("unknown")),
        }
    }
}
