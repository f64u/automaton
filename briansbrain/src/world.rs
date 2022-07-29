use cellular_automaton::{
    cell::BasicCell,
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
    fn changes(&self) -> Vec<(Index, Cell)> {
        let mut delta = vec![];

        for p @ (i, j) in (0..self.dimensions().0).cartesian_product(0..self.dimensions().1) {
            let cell = &self.cells()[j][i];
            let alive = self
                .moore_neighbors(p)
                .iter()
                .filter(|c| matches!(***c, Cell::On))
                .count();

            match cell {
                Cell::Off if 2 == alive => delta.push(((i, j), cell.next_state())),
                Cell::Dying => delta.push(((i, j), cell.next_state())),
                Cell::On => delta.push(((i, j), cell.next_state())),
                _ => {}
            }
        }

        delta
    }

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
