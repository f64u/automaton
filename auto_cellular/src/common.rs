/// Screen position of a physical space
pub type ScreenPosition = (isize, isize);
/// Index of a cell in a world
pub type Index = (usize, usize);

/// Dimensions of a world
#[derive(Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);
/// A grid
pub type DoubleVec<T> = Vec<Vec<T>>;

pub fn linearize<T>(vector: DoubleVec<T>) -> Vec<(Index, T)> {
    vector
        .into_iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(i, cell)| ((i, j), cell))
        })
        .collect()
}
