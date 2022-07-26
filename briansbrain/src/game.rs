use cellular_automaton::{
    cell::BasicCell,
    common::{Grid, Index},
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
    fn next(&self) -> Self {
        match *self {
            Cell::On => Cell::Dying,
            Cell::Dying => Cell::Off,
            Cell::Off => Cell::On,
        }
    }
}

pub fn random_cells(dimensions: (usize, usize), p: f64) -> impl Iterator<Item = Cell> {
    (0..dimensions.0 * dimensions.1).map(move |_| {
        let x: f64 = rand::random();
        if x < p {
            Cell::Off
        } else {
            Cell::On
        }
    })
}

pub struct World<const W: usize, const H: usize> {
    cells: Grid<Cell, W, H>,
}

impl<const W: usize, const H: usize> BasicWorld<W, H> for World<W, H> {
    type Cell = Cell;

    fn new(cells: impl Iterator<Item = Self::Cell>) -> Self {
        let default = Cell::default();
        let mut acells = [[default; W]; H];
        for (y, row) in cells.chunks(W).into_iter().enumerate() {
            for (x, cell) in row.enumerate() {
                acells[y][x] = cell;
            }
        }
        Self { cells: acells }
    }

    fn new_random() -> Self {
        Self::new(random_cells((W, H), PROPORTION))
    }

    fn refresh_random(&mut self) {
        *self = Self::new(random_cells((W, H), PROPORTION))
    }

    fn cells(&self) -> &Grid<Cell, W, H> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut Grid<Cell, W, H> {
        &mut self.cells
    }

    fn delta_future(&self) -> Vec<(Index, Self::Cell)> {
        let mut delta = vec![];

        for (i, j) in (0..W).cartesian_product(0..H) {
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
                Cell::Off if 2 == alive => delta.push(((i, j), cell.next())),
                Cell::Dying => delta.push(((i, j), cell.next())),
                Cell::On => delta.push(((i, j), cell.next())),
                _ => {}
            }
        }

        delta
    }
}
