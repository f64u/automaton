use cellular_automaton::cell::BasicCell;

use crate::PROPORTION;

#[derive(Clone, Default, Copy)]
pub enum Cell {
    On,
    Dying,
    #[default]
    Off,
}

impl BasicCell for Cell {
    fn next_state(&self) -> Self {
        match *self {
            Cell::On => Cell::Dying,
            Cell::Dying => Cell::Off,
            Cell::Off => Cell::On,
        }
    }
    fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let x: f64 = rng.gen();
        if x < PROPORTION {
            Cell::Off
        } else {
            Cell::On
        }
    }
}
