use crate::{
    cell::BasicCell,
    common::{Dimensions, DoubleVec, Index},
};

pub trait WorldConfig {
    fn dimensions(&self) -> Dimensions;
}

/// A given [BasicWorld] knows how to go from one state of [BasicCell] to the next on each
/// tick by provding a [BasicWorld::changes] method
pub trait BasicWorld
where
    Self: Sized,
{
    /// Type of [BasicCell] this world manages
    type Cell: BasicCell;
    /// Type of [WorldConfig] this world takes
    type Config: WorldConfig;

    /// Gives a blank [BasicWorld] of the same configuration as the original world
    fn blank(&self) -> Self {
        Self::new_blank(self.config())
    }

    /// Gives a random [BasicWorld] of the same configuration as the original world
    fn random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
        Self::new_random(rng, self.config())
    }

    /// Constaruct a new [BasicWorld]
    fn new(cells: DoubleVec<Self::Cell>, config: Self::Config) -> Self;
    fn new_blank(config: Self::Config) -> Self {
        let default = Self::Cell::default();
        let cells = vec![vec![(); config.dimensions().0]; config.dimensions().1]
            .into_iter()
            .map(|row| row.into_iter().map(|_| default).collect())
            .collect();
        Self::new(cells, config)
    }

    fn new_random<R: rand::Rng + ?Sized>(rng: &mut R, config: Self::Config) -> Self {
        let cells = (0..config.dimensions().0 * config.dimensions().1)
            .collect::<Vec<usize>>()
            .chunks(config.dimensions().0)
            .into_iter()
            .map(|chunk| chunk.iter().map(|_| Self::Cell::random(rng)).collect())
            .collect();
        Self::new(cells, config)
    }

    /// Gets a shared referene to the grid of [BasicCell]s
    fn cells(&self) -> &DoubleVec<Self::Cell>;
    /// Gets a mutable referene to the grid of [BasicCell]s
    fn cells_mut(&mut self) -> &mut DoubleVec<Self::Cell>;

    /// Given a [BasicWorld], return the changes to [BasicCell]s that
    /// would happned the *upcoming* tick and the [Index]s where they happened
    fn changes(&self) -> Vec<(Index, Self::Cell)>;

    /// Returns the dela that happened the previous [BasicWorld::tick]
    fn delta(&self) -> &Vec<(Index, Self::Cell)>;
    /// Returns a mutable reference to that value
    fn delta_mut(&mut self) -> &mut Vec<(Index, Self::Cell)>;

    /// Get the dimensions of the world
    fn config(&self) -> Self::Config;

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
        let (w, h) = (self.config().dimensions().0 as isize, self.config().dimensions().1 as isize);

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
    struct Config;
    
    impl WorldConfig for Config {
        fn dimensions(&self) -> Dimensions {
            Dimensions(50, 50)
        }
    }

    impl BasicWorld for World {
        type Cell = Cell;
        type Config = Config;

        fn new(_cells: DoubleVec<Cell>, _config: Config) -> Self {
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

        fn config(&self) -> Config {
            Config
        }
    }

    #[test]
    fn moore_neighbors_works() {
        let world = World;

        println!("{:?}", world.moore_neighbors((49, 49)));
    }
}
