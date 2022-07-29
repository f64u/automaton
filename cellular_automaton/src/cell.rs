use rand::prelude::*;

/// A cell that knows how to get its next state and a random verion of its type
pub trait BasicCell: Default + Clone + Copy {
    fn next_state(&self) -> Self;
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}
