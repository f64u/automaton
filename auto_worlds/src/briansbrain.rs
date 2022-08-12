use auto_cellular::{
    cell::BasicCell,
    common::{linearize, Dimensions, DoubleVec, Index},
    world::BasicWorld,
};

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

pub struct World {
    cells: DoubleVec<Cell>,
    dimensions: Dimensions,
    delta: Vec<(Index, Cell)>,
}

impl BasicWorld for World {
    type Cell = Cell;

    fn changes(&self) -> Vec<(Index, Cell)> {
        let mut delta = vec![];

        for j in 0..self.dimensions().1 {
            for i in 0..self.dimensions().0 {
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

    fn new(cells: DoubleVec<Cell>, dimensions: Dimensions) -> Self {
        let clone = cells.clone();
        Self {
            cells,
            dimensions,
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

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}
