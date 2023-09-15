use auto_cellular::{
    cell::CellLike,
    common::{linearize, Dimensions, DoubleVec, Index},
    world::{WorldConfig, WorldLike},
};

use crate::PROPORTION;

#[derive(Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Alive,

    #[default]
    Dead,
}

impl CellLike for Cell {
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

#[derive(Clone)]
pub struct WConfig {
    pub dimensions: Dimensions,
}

impl WorldConfig for WConfig {
    fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
}

pub struct World {
    cells: DoubleVec<Cell>,
    config: WConfig,
    delta: Vec<(Index, Cell)>,
}

impl WorldLike for World {
    type Cell = Cell;
    type Config = WConfig;

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

    fn changes(&self) -> Vec<(Index, Cell)> {
        let mut delta = vec![];

        for j in 0..self.config().dimensions().1 {
            for i in 0..self.config().dimensions().0 {
                let p = (i, j);
                let count = self
                    .moore_neighbors(p)
                    .iter()
                    .filter(|c| matches!(self.cells[c.1][c.0], Cell::Alive))
                    .count();
                let cell = &self.cells()[j][i];
                match *cell {
                    Cell::Alive if !(2..=3).contains(&count) => delta.push((p, Cell::Dead)),

                    Cell::Dead if count == 3 => delta.push((p, Cell::Alive)),
                    _ => {}
                }
            }
        }
        delta
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
