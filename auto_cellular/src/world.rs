use crate::{
    cell::BasicCell,
    common::{Dimensions, DoubleVec, Index},
};

/// A given [BasicWorld] knows how to go from one state of [BasicCell] to the next on each
/// tick by provding a [BasicWorld::changes] method
pub trait BasicWorld<C>
where
    C: BasicCell,
    Self: Sized,
{
    /// Gives a blank [BasicWorld] of the same configuration as the original world
    fn blank(&self) -> Self {
        Self::new_blank(self.dimensions())
    }

    /// Gives a random [BasicWorld] of the same configuration as the original world
    fn random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        Self::new_random(rng, self.dimensions())
    }

    /// Constaruct a new [BasicWorld]
    fn new(cells: DoubleVec<C>, dimensions: Dimensions) -> Self;
    fn new_blank(dimensions: Dimensions) -> Self {
        let default = C::default();
        let cells = vec![vec![(); dimensions.0]; dimensions.1]
            .into_iter()
            .map(|row| row.into_iter().map(|_| default).collect())
            .collect();
        Self::new(cells, dimensions)
    }

    fn new_random<R: rand::Rng + ?Sized>(rng: &mut R, dimensions: Dimensions) -> Self {
        let cells = (0..dimensions.0 * dimensions.1)
            .collect::<Vec<usize>>()
            .chunks(dimensions.0)
            .into_iter()
            .map(|chunk| chunk.iter().map(|_| C::random(rng)).collect())
            .collect();
        Self::new(cells, dimensions)
    }

    /// Gets a shared referene to the grid of [BasicCell]s
    fn cells(&self) -> &DoubleVec<C>;
    /// Gets a mutable referene to the grid of [BasicCell]s
    fn cells_mut(&mut self) -> &mut DoubleVec<C>;

    /// Given a [BasicWorld], return the changes to [BasicCell]s that
    /// would happned the *upcoming* tick and the [Index]s where they happened
    fn changes(&self) -> Vec<(Index, C)>;

    /// Returns the dela that happened the previous [BasicWorld::tick]
    fn delta(&self) -> &Vec<(Index, C)>;
    /// Returns a mutable reference to that value
    fn delta_mut(&mut self) -> &mut Vec<(Index, C)>;

    /// Get the dimensions of the world
    fn dimensions(&self) -> Dimensions;

    /// Commit the [BasicWorld::changes] to memory
    fn tick(&mut self) {
        let changes = self.changes();
        for ((x, y), cell) in changes.iter() {
            self.cells_mut()[*y][*x] = *cell;
        }
        *self.delta_mut() = changes;
    }

    /// A click happened at a given [Index]
    fn click(&mut self, i @ (x, y): Index) {
        let c = &mut self.cells_mut()[y][x];
        *c = c.next_state();
        *self.delta_mut() = vec![(i, *c)]
    }

    /// Returns the Moore Neihgbors for a given [BasicCell] at a given [Index] (x, y)
    fn moore_neighbors(&self, p @ (x, y): Index) -> Vec<Index> {
        let (x, y) = (x as isize, y as isize);
        let (w, h) = (self.dimensions().0 as isize, self.dimensions().1 as isize);

        (-1..=1)
            .flat_map(|i| {
                (-1..=1).map(move |j| {
                    (
                        (x + i).rem_euclid(w) as usize,
                        (y + j).rem_euclid(h) as usize,
                    )
                })
            })
            .filter(move |&item| item != p)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cell::test::Cell;

    struct World;

    impl BasicWorld<Cell> for World {
        fn new(cells: DoubleVec<Cell>, dimensions: Dimensions) -> Self {
            todo!()
        }

        fn cells(&self) -> &DoubleVec<Cell> {
            todo!()
        }

        fn cells_mut(&mut self) -> &mut DoubleVec<Cell> {
            todo!()
        }

        fn changes(&self) -> Vec<(Index, Cell)> {
            todo!()
        }

        fn delta(&self) -> &Vec<(Index, Cell)> {
            todo!()
        }

        fn delta_mut(&mut self) -> &mut Vec<(Index, Cell)> {
            todo!()
        }

        fn dimensions(&self) -> Dimensions {
            Dimensions(50, 50)
        }
    }

    #[test]
    fn moore_neighbors_works() {
        let world = World;

        println!("{:?}", world.moore_neighbors((49, 49)));
    }
}
