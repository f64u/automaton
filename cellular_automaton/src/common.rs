pub trait RepresentableAs<T> {
    type Delta;
    fn represent(&self) -> T;
    fn next_frame(&self) -> Self::Delta;
}

pub type Position = (isize, isize);
pub type Index = (usize, usize);
pub type Grid<T, const W: usize, const H: usize> = [[T; W]; H];
