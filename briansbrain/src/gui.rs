use cellular_automaton::{common::RepresentableAs, world::BasicWorld};
use sdl2::pixels::Color;
use spaces::sdl2_canvas::{self, ColoredCell, ColoredWorld, Config, FutureWorld, RepresentedWorld};

use crate::game::{Cell, World};

impl RepresentableAs<Color> for Cell {
    type Delta = Color;
    fn represent(&self) -> Color {
        match *self {
            Cell::Dying => Color::BLUE,
            Cell::On => Color::WHITE,
            Cell::Off => Color::BLACK,
        }
    }

    fn next_frame(&self) -> Self::Delta {
        RepresentableAs::<Self::Delta>::represent(self)
    }
}

impl ColoredCell for Cell {}

impl<const W: usize, const H: usize> RepresentableAs<RepresentedWorld<W, H>> for World<W, H> {
    type Delta = FutureWorld;
    fn represent(&self) -> RepresentedWorld<W, H> {
        ColoredWorld::represent(self)
    }

    fn next_frame(&self) -> Self::Delta {
        ColoredWorld::next_frame(self)
    }
}

impl<const W: usize, const H: usize> ColoredWorld<W, H> for World<W, H> {}

pub(crate) fn run() -> Result<(), String> {
    const WINDOW_W: usize = 1200;
    const WINDOW_H: usize = 700;
    const PIXEL_SIZE: usize = 20;
    const W: usize = WINDOW_W / PIXEL_SIZE;
    const H: usize = WINDOW_H / PIXEL_SIZE;
    let config = Config::new((WINDOW_W as u32, WINDOW_H as u32), PIXEL_SIZE, 100);
    let world = World::<W, H>::new_random();
    sdl2_canvas::run(config, world, "Game of Life")?;

    Ok(())
}
