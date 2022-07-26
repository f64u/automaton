use cellular_automaton::{
    common::{Dimensions, RepresentableAs},
    world::BasicWorld,
};
use sdl2::pixels::Color;
use spaces::sdl2_canvas::{self, ColoredCell, ColoredWorld, Config, RepresentedWorld};

use crate::game::{Cell, World};

impl RepresentableAs<Color> for Cell {
    fn represent(&self) -> Color {
        match *self {
            Cell::Dying => Color::BLUE,
            Cell::On => Color::WHITE,
            Cell::Off => Color::BLACK,
        }
    }
}

impl ColoredCell for Cell {}

impl RepresentableAs<RepresentedWorld> for World {
    fn represent(&self) -> RepresentedWorld {
        ColoredWorld::represent(self)
    }
}

impl ColoredWorld for World {}

pub(crate) fn run() -> Result<(), String> {
    let config = Config::new(Dimensions(1000, 800), 20, 100);
    let world = World::new_random(Dimensions(config.pixel_count_x(), config.pixel_count_y()));
    sdl2_canvas::run(config, world, "Brian's Brain")?;

    Ok(())
}
