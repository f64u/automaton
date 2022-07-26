use itertools::Itertools;

use crate::{
    cell::BasicCell,
    common::{Grid, Index, Position},
};

pub trait BasicWorld<const W: usize, const H: usize> {
    type Cell: BasicCell;

    fn new_random() -> Self;
    fn new(cells: impl Iterator<Item = Self::Cell>) -> Self;
    fn refresh_random(&mut self);
    fn cells(&self) -> &Grid<Self::Cell, W, H>;
    fn cells_mut(&mut self) -> &mut Grid<Self::Cell, W, H>;
    fn delta_future(&self) -> Vec<(Index, Self::Cell)>;
    fn tick(&mut self) {
        for ((x, y), cell) in self.delta_future() {
            self.cells_mut()[y][x] = cell;
        }
    }

    fn blank(&mut self) {
        let default = Self::Cell::default();
        *self.cells_mut() = [[default; W]; H];
    }

    fn moore_neighbors(&self, (x, y): Position) -> Vec<&Self::Cell> {
        (x.max(1) - 1..=(x + 1).min(W as isize - 1))
            .cartesian_product(y.max(1) - 1..=(y + 1).min(H as isize - 1))
            .filter(move |&item| item != (x, y))
            .map(|(x, y)| &self.cells()[y as usize][x as usize])
            .collect()
    }
}
