use cellular_automaton::{
    common::{Dimensions, Repr},
    world::BasicWorld,
};
use sdl2::pixels::Color;
use spaces::sdl2_canvas::{self, Config};

use crate::game::{Cell, World};

impl Repr<Color> for Cell {
    fn repr(&self) -> Color {
        match self {
            Cell::Alive => Color::WHITE,
            Cell::Dead => Color::BLACK,
        }
    }
}

pub(crate) fn run() -> Result<(), String> {
    const WINDOW_W: usize = 1200;
    const WINDOW_H: usize = 700;
    const PIXEL_SIZE: usize = 20;
    const W: usize = WINDOW_W / PIXEL_SIZE;
    const H: usize = WINDOW_H / PIXEL_SIZE;
    let config = Config::new((WINDOW_W as u32, WINDOW_H as u32), PIXEL_SIZE as usize, 100);
    let mut rng = rand::thread_rng();
    let world = World::random(&mut rng, Dimensions(W, H));
    sdl2_canvas::run(config, world, "Game of Life")?;

    Ok(())
}
