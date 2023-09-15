use auto_spaces::SpaceKind;
use auto_worlds::WorldKind;
use clap::Parser;

#[cfg(feature = "sdl2")]
pub mod gui;
#[cfg(feature = "cursive")]
pub mod terminal;
#[cfg(feature = "wasm")]
pub mod web;

/// Simulate basic cellular automaton-based worlds
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the canvas to draw in
    #[clap(short = 'W', long, value_parser, default_value_t = 1200)]
    width: usize,

    /// Height of the canvas to draw in
    #[clap(short = 'H', long, value_parser, default_value_t = 700)]
    height: usize,

    /// The size of each cell represented in the canvas
    #[clap(short, long, value_parser, default_value_t = 20)]
    cell_size: usize,

    /// Time to wait between each tick
    #[clap(short, long, value_parser, default_value_t = 100)]
    update_millis: usize,

    /// The type of [Worlds] to simulate
    #[clap(short, long)]
    world: WorldKind,

    /// the type of [Space] to use as canvas
    #[clap(short, long)]
    frontend: SpaceKind,
}

fn main() -> Result<(), String> {
    use auto_cellular::common::Dimensions;

    let args = Args::parse();

    match args.frontend {
        #[cfg(feature = "cursive")]
        SpaceKind::Cursive => terminal::run(
            args.world,
            Dimensions(args.width, args.height),
            args.update_millis,
        ),
        #[cfg(feature = "sdl2")]
        SpaceKind::Sdl2 => gui::run(
            args.world,
            Dimensions(args.width, args.height),
            args.cell_size,
            args.update_millis,
        ),
    }
}
