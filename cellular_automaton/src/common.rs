pub type ScreenPosition = (isize, isize);
pub type Index = (usize, usize);
pub trait Repr<T> {
    fn repr(&self) -> T;
}

#[derive(Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);
pub type DoubleVec<T> = Vec<Vec<T>>;
