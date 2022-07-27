use cellular_automaton::cell::BasicCell;

use crate::PROPORTION;

#[derive(Clone, Default, Copy)]
pub enum Cell {
    Alive,

    #[default]
    Dead,
}

impl BasicCell for Cell {
    fn next_state(&self) -> Self {
        match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }

    fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let f: f64 = rng.gen();
        if f < PROPORTION {
            Cell::Dead
        } else {
            Cell::Alive
        }
    }
}
