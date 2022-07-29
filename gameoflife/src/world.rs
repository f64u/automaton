use cellular_automaton::{
    common::{Dimensions, DoubleVec, Index},
    world::BasicWorld,
};
use itertools::Itertools;

use crate::cell::Cell;

pub struct World {
    cells: DoubleVec<Cell>,
    dimensions: Dimensions,
    delta: Vec<(Index, Cell)>,
}

impl BasicWorld<Cell> for World {
    fn new(cells: DoubleVec<Cell>, dimensions: Dimensions) -> Self {
        let clone = cells.clone();
        Self {
            cells,
            dimensions,
            delta: clone
                .into_iter()
                .enumerate()
                .flat_map(|(j, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(move |(i, cell)| ((i, j), cell))
                })
                .collect(),
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

        for p @ (i, j) in (0..self.dimensions().0).cartesian_product(0..self.dimensions().1) {
            let count = self
                .moore_neighbors(p)
                .iter()
                .filter(|c| matches!(***c, Cell::Alive))
                .count();
            let cell = &self.cells()[j][i];
            match *cell {
                Cell::Alive if !(2..=3).contains(&count) => delta.push(((i, j), Cell::Dead)),

                Cell::Dead if count == 3 => delta.push(((i, j), Cell::Alive)),
                _ => {}
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

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}
