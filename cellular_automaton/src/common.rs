/// Screen position of a physical space
pub type ScreenPosition = (isize, isize);
/// Index of a cell in a world
pub type Index = (usize, usize);

/// Dimensions of a world
#[derive(Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);
/// A grid
pub type DoubleVec<T> = Vec<Vec<T>>;
