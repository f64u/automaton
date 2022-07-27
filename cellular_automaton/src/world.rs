use itertools::Itertools;

use crate::{
    cell::BasicCell,
    common::{Dimensions, DoubleVec, Index, ScreenPosition},
};

pub trait BasicWorld<C>
where
    C: BasicCell,
    Self: Sized,
{
    fn random<R: rand::Rng + ?Sized>(rng: &mut R, dimensions: Dimensions) -> Self {
        let cells = (0..dimensions.0 * dimensions.1)
            .chunks(dimensions.0)
            .into_iter()
            .map(|chunk| chunk.map(|_| C::random(rng)).collect())
            .collect();
        Self::new(cells, dimensions)
    }
    fn new(cells: DoubleVec<C>, dimensions: Dimensions) -> Self;

    fn cells(&self) -> &DoubleVec<C>;
    fn cells_mut(&mut self) -> &mut DoubleVec<C>;

    /// Not to be called directly.
    fn changes(&self) -> Vec<(Index, C)>;

    // Tick handles updating the deltas
    fn delta(&self) -> &Vec<(Index, C)>;
    fn delta_mut(&mut self) -> &mut Vec<(Index, C)>;

    fn dimensions(&self) -> Dimensions;

    fn tick(&mut self) {
        let changes = self.changes();
        for ((x, y), cell) in changes.iter() {
            self.cells_mut()[*y][*x] = cell.clone();
        }
        *self.delta_mut() = changes;
    }

    fn blank(&mut self) {
        let default = C::default();
        *self.cells_mut() = vec![vec![(); self.dimensions().0]; self.dimensions().1]
            .into_iter()
            .map(|row| row.into_iter().map(|_| default).collect())
            .collect();
    }

    fn moore_neighbors(&self, (x, y): ScreenPosition) -> Vec<&C> {
        (x.max(1) - 1..=(x + 1).min(self.dimensions().0 as isize - 1))
            .cartesian_product(y.max(1) - 1..=(y + 1).min(self.dimensions().1 as isize - 1))
            .filter(move |&item| item != (x, y))
            .map(|(x, y)| &self.cells()[y as usize][x as usize])
            .collect()
    }
}
