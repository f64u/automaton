use rand::prelude::*;

pub trait BasicCell: Default + Clone + Copy {
    fn next_state(&self) -> Self;
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}
