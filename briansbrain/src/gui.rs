use cellular_automaton::{
    common::{Dimensions, Representable},
    world::BasicWorld,
};
use sdl2::pixels::Color;
use spaces::sdl_canvas::{self, ColoredCell, ColoredWorld, Config, RepresentedWorld};

use crate::game::{Cell, World};

impl Representable<Color> for Cell {
    fn represent(&self) -> Color {
        match *self {
            Cell::Dying => Color::RGB(0, 0, 255),
            Cell::On => Color::WHITE,
            Cell::Off => Color::BLACK,
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
    let world = World::new_random(Dimensions(1000 / 5, 800 / 5));
    sdl_canvas::run(config, world, "Brian's Brain")?;

    Ok(())
}
