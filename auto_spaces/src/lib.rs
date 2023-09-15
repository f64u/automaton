use std::{fmt::Display, str::FromStr};

pub mod common;
#[cfg(feature = "cursive")]
pub mod cursive_canvas;
#[cfg(feature = "sdl2")]
pub mod sdl2_canvas;

#[cfg(feature = "wasm")]
pub mod wasm_canvas;

#[derive(Debug)]
pub enum SpaceKind {
    #[cfg(feature = "sdl2")]
    Sdl2,

    #[cfg(feature = "cursive")]
    Cursive,
}

impl Display for SpaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for SpaceKind {
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
