pub type ScreenPosition = (isize, isize);
pub type Index = (usize, usize);

#[derive(Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);
pub type DoubleVec<T> = Vec<Vec<T>>;
