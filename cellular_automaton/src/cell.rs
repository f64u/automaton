pub trait BasicCell: Default + Clone + Copy {
    fn next(&self) -> Self;
}
