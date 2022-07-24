pub trait BasicCell: Default + Clone {
    fn next(&mut self);
}
