use auto_cellular::{
    cell::CellLike,
    common::{linearize, Dimensions, DoubleVec, Index},
    world::{WorldConfig, WorldLike},
};

use crate::PROPORTION;

#[derive(Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    On,
    Dying,
    #[default]
    Off,
}

impl CellLike for Cell {
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

pub struct World {
    cells: DoubleVec<Cell>,
    config: WConfig,
    delta: Vec<(Index, Cell)>,
}

#[derive(Clone)]
pub struct WConfig {
    pub dimensions: Dimensions,
}

impl WorldConfig for WConfig {
    fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
}

impl WorldLike for World {
    type Cell = Cell;
    type Config = WConfig;

    fn changes(&self) -> Vec<(Index, Cell)> {
        let mut delta = vec![];

        for j in 0..self.config().dimensions().1 {
            for i in 0..self.config().dimensions().0 {
                let p = (i, j);
                let cell = &self.cells()[j][i];
                let alive = self
                    .moore_neighbors(p)
                    .iter()
                    .filter(|c| matches!(self.cells()[c.1][c.0], Cell::On))
                    .count();

                match cell {
                    Cell::Off if 2 == alive => delta.push((p, cell.next_state())),
                    Cell::Dying => delta.push((p, cell.next_state())),
                    Cell::On => delta.push((p, cell.next_state())),
                    _ => {}
                }
            }
        }

        delta
    }

    fn new(cells: DoubleVec<Cell>, config: Self::Config) -> Self {
        let clone = cells.clone();
        Self {
            cells,
            config,
            delta: linearize(clone),
        }
    }

    fn cells(&self) -> &DoubleVec<Cell> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut DoubleVec<Cell> {
        &mut self.cells
    }

    fn delta(&self) -> &Vec<(Index, Cell)> {
        &self.delta
    }

    fn delta_mut(&mut self) -> &mut Vec<(Index, Cell)> {
        &mut self.delta
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }
}
