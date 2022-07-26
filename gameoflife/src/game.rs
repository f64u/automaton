use cellular_automaton::{
    cell::BasicCell,
    common::{Grid, Index, Position},
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
    fn next(&mut self) {
        std::mem::swap(
            self,
            &mut match *self {
                Cell::Alive => Cell::Dead,
                Cell::Dead => Cell::Alive,
            },
        )
    }
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            _ => false,
        }
    }
}

pub fn random_cells(dimensions: (usize, usize), p: f64) -> impl Iterator<Item = Cell> {
    (0..dimensions.0 * dimensions.1).map(move |_| {
        let x: f64 = rand::random();
        if x < p {
            Cell::Dead
        } else {
            Cell::Alive
        }
    })
}

pub struct World<const W: usize, const H: usize> {
    cells: Grid<Cell, W, H>,
}

impl<const W: usize, const H: usize> BasicWorld<W, H> for World<W, H> {
    type Cell = Cell;

    fn cells(&self) -> &Grid<Cell, W, H> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut Grid<Cell, W, H> {
        &mut self.cells
    }

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

    fn delta_future(&self) -> Vec<(Index, Self::Cell)> {
        let mut delta = vec![];

        for (i, j) in (0..W).cartesian_product(0..H) {
            let p = (i as isize, j as isize);
            let count = self.count_alive_neighbors(p);
            let cell = &self.cells()[j][i];
            match cell {
                &Cell::Alive if count < 2 || count > 3 => delta.push(((i, j), Cell::Dead)),

                &Cell::Dead if count == 3 => delta.push(((i, j), Cell::Alive)),
                _ => {}
            }
        }
        delta
    }

    fn refresh_random(&mut self) {
        *self = Self::new(random_cells((W, H), PROPORTION))
    }
}

impl<const W: usize, const H: usize> World<W, H> {
    fn count_alive_neighbors(&self, p: Position) -> usize {
        self.moore_neighbors(p)
            .iter()
            .filter(|c| c.is_alive())
            .count()
    }
}
