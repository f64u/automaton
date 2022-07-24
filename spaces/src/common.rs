use std::ops::{Deref, DerefMut};

pub struct Output<Field>(pub Field);

impl<Field> Deref for Output<Field> {
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Field> DerefMut for Output<Field> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
