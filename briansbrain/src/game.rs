use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, DoubleVec, Index},
    world::BasicWorld,
};
use itertools::Itertools;

pub const PROPORTION: f64 = 0.9;

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

impl BasicWorld<Cell> for World {
    fn changes(&self) -> Vec<(Index, Cell)> {
        let mut delta = vec![];

        for (i, j) in (0..self.dimensions().0).cartesian_product(0..self.dimensions().1) {
            let p = (i as isize, j as isize);
            let cell = &self.cells()[j][i];
            let alive = self
                .moore_neighbors(p)
                .iter()
                .filter(|c| match ***c {
                    Cell::On => true,
                    _ => false,
                })
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
