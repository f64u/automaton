use cellular_automaton::{
    common::{Dimensions, Representable},
    world::BasicWorld,
};
use sdl2::pixels::Color;
use spaces::sdl_canvas::{self, ColoredCell, ColoredWorld, Config, RepresentedWorld};

use crate::game::{Cell, World};

impl Representable<Color> for Cell {
    fn represent(&self) -> Color {
        if self.is_alive() {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
}

impl ColoredCell for Cell {}

impl Representable<RepresentedWorld> for World {
    fn represent(&self) -> RepresentedWorld {
        ColoredWorld::represent(self)
    }
}

impl ColoredWorld for World {}

pub(crate) fn run() -> Result<(), String> {
    let config = Config::new(Dimensions(1000, 800), 5);
    let world = World::new_random(Dimensions(config.pixel_count_x(), config.pixel_count_y()));
    sdl_canvas::run(config, world, "Game of Life")?;

    Ok(())
}
