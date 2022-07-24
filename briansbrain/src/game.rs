use cellular_automaton::{cell::BasicCell, common::Dimensions, world::BasicWorld};

pub const PROPORTION: f64 = 0.9;

pub enum Cell {
    On,
    Dying,
    Off,
}

impl BasicCell for Cell {}

impl Cell {
    pub fn shoot(&mut self) {
        *self = Cell::Dying;
    }

    pub fn kill(&mut self) {
        *self = Cell::Off;
    }

    pub fn resurrect(&mut self) {
        *self = Cell::On;
    }
}

pub fn random_cells(dimensions: Dimensions, p: f64) -> impl Iterator<Item = Cell> {
    (0..dimensions.0 * dimensions.1).map(move |_| {
        let x: f64 = rand::random();
        if x < p {
            Cell::Off
        } else {
            Cell::On
        }
    })
}

pub struct World {
    cells: Vec<Cell>,
    dimensions: Dimensions,
}

impl BasicWorld for World {
    type Cell = Cell;

    fn new(dimensions: Dimensions, initial_cells: Vec<Self::Cell>) -> Self {
        Self {
            cells: initial_cells,
            dimensions,
        }
    }

    fn new_random(dimensions: Dimensions) -> Self {
        Self::new(dimensions, random_cells(dimensions, PROPORTION).collect())
    }

    fn refresh_random(&mut self) {
        *self.cells_mut() = random_cells(self.dimensions(), PROPORTION).collect();
    }

    fn cells(&self) -> &Vec<Self::Cell> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut Vec<Self::Cell> {
        &mut self.cells
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    fn tick(&mut self) {
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                let p = (i as isize, j as isize);
                let alive = self
                    .moore_neighbors(p)
                    .iter()
                    .filter(|c| match ***c {
                        Cell::On => true,
                        _ => false,
                    })
                    .count();
                let cell = self.get_cell_mut(p).unwrap();
                match *cell {
                    Cell::Off if alive == 2 => cell.resurrect(),
                    Cell::Dying => cell.kill(),
                    Cell::On => cell.shoot(),
                    _ => {}
                }
            }
        }
    }
}
