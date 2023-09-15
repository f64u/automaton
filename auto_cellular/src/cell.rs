use std::hash::Hash;

use rand::Rng;

/// A cell that knows how to get its next state and a random verion of its type
pub trait CellLike: Default + Clone + Copy + Hash + Eq {
    fn next_state(&self) -> Self;
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[derive(Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
    pub(crate) struct Cell;

    impl CellLike for Cell {
        fn next_state(&self) -> Self {
            Cell
        }

        fn random<R: Rng + ?Sized>(_rng: &mut R) -> Self {
            Cell
        }
    }
}
