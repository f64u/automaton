use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, DoubleVec, Index},
    world::BasicWorld,
};
use itertools::Itertools;

const PROPORTION: f64 = 0.9;

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

        for (i, j) in (0..self.dimensions().0).cartesian_product(0..self.dimensions().1) {
            let p = (i as isize, j as isize);
            let count = self
                .moore_neighbors(p)
                .iter()
                .filter(|c| match ***c {
                    Cell::Alive => true,
                    _ => false,
                })
                .count();
            let cell = &self.cells()[j][i];
            match cell {
                &Cell::Alive if count < 2 || count > 3 => delta.push(((i, j), Cell::Dead)),

                &Cell::Dead if count == 3 => delta.push(((i, j), Cell::Alive)),
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
