pub trait BasicCell: Default + Clone + Copy {
    fn next(&mut self);
}
