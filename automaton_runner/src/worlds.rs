use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Worlds {
    GameOfLife,
    BriansBrain,
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
            _ => Err(String::from("unknown")),
        }
    }
}

#[derive(Debug)]
pub enum FrontEnds {
    #[cfg(feature = "sdl2")]
    Sdl2,

    #[cfg(feature = "cursive")]
    Cursive,
}

impl Display for FrontEnds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for FrontEnds {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_lowercase()[..] {
            #[cfg(feature = "sdl2")]
            "gui" | "sdl" | "sdl2" => Ok(Self::Sdl2),

            #[cfg(feature = "cursive")]
            "terminal" | "cursive" => Ok(Self::Cursive),
            _ => Err(String::from("unknown")),
        }
    }
}
