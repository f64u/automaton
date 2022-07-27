use cellular_automaton::{
    common::{Dimensions, Repr},
    world::BasicWorld,
};
use spaces::cursive_canvas;

use crate::game::{Cell, World};

impl Repr<char> for Cell {
    fn repr(&self) -> char {
        match *self {
            Cell::Alive => '#',
            Cell::Dead => ' ',
        }
    }
}

pub(crate) fn run() -> Result<(), String> {
    const W: usize = 70;
    const H: usize = 70;

    let mut rng = rand::thread_rng();
    let world = World::random(&mut rng, Dimensions(W, H));
    cursive_canvas::run(world)?;

    Ok(())
}
