use crate::worlds::{FrontEnds, Worlds};
use clap::Parser;

#[cfg(feature = "sdl2")]
mod gui;
#[cfg(feature = "cursive")]
mod terminal;
#[cfg(feature = "wasm")]
mod web;

mod worlds;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = 1200)]
    width: usize,

    #[clap(short, long, value_parser, default_value_t = 700)]
    height: usize,

    #[clap(short, long, value_parser, default_value_t = 20)]
    pixel_size: usize,

    #[clap(short, long, value_parser, default_value_t = 100)]
    update_millis: usize,

    #[clap(short, long, default_value_t = Worlds::GameOfLife)]
    game: Worlds,

    #[clap(short, long, default_value_t = FrontEnds::Sdl2)]
    frontend: FrontEnds,
}

#[cfg(all(feature = "sdl2", feature = "cursive"))]
fn main() -> Result<(), String> {
    use cellular_automaton::common::Dimensions;

    let args = Args::parse();

    match args.frontend {
        FrontEnds::Curisve => terminal::run(
            args.game,
            Dimensions(args.width, args.height),
            args.update_millis,
        ),
        FrontEnds::Sdl2 => gui::run(
            worlds::Worlds::BriansBrain,
            Dimensions(args.width, args.height),
            args.pixel_size,
            args.update_millis,
        ),
    }
}

#[cfg(all(feature = "sdl2", not(feature = "cursive")))]
fn main() -> Result<(), String> {
    gui::run()?;
    Ok(())
}

#[cfg(all(feature = "cursive", not(feature = "sdl2")))]
fn main() -> Result<(), String> {
    terminal::run()?;
    Ok(())
}

#[cfg(not(any(feature = "cursive", feature = "sdl2")))]
fn main() {}
