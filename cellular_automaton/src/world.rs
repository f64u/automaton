use itertools::Itertools;

use crate::{
    cell::BasicCell,
    common::{Dimensions, Position},
};

pub trait BasicWorld {
    type Cell: BasicCell;

    fn new(dimensions: Dimensions, initial_cells: Vec<Self::Cell>) -> Self;
    fn new_random(dimensions: Dimensions) -> Self;
    fn refresh_random(&mut self);
    fn cells(&self) -> &Vec<Self::Cell>;
    fn cells_mut(&mut self) -> &mut Vec<Self::Cell>;
    fn dimensions(&self) -> Dimensions;
    fn next(&self) -> Vec<Self::Cell>;
    fn tick(&mut self) {
        *self.cells_mut() = self.next();
    }

    fn blank(&mut self) {
        *self.cells_mut() = vec![Self::Cell::default(); self.dimensions().size()];
    }

    fn get_cell(&self, p: Position) -> Option<&Self::Cell> {
        self.dimensions()
            .get_index(p)
            .map(|index| &self.cells()[index])
    }

    fn get_cell_mut(&mut self, p: Position) -> Option<&mut Self::Cell> {
        self.dimensions()
            .get_index(p)
            .map(|index| &mut self.cells_mut()[index])
    }

    fn moore_neighbors(&self, (x, y): Position) -> Vec<&Self::Cell> {
        (x.max(1) - 1..=(x + 1).min(self.dimensions().0 as isize - 1))
            .cartesian_product(y.max(1) - 1..=(y + 1).min(self.dimensions().1 as isize - 1))
            .filter(move |&item| item != (x, y))
            .filter_map(|p| self.get_cell(p))
            .collect()
    }
}
