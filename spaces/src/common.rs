use std::ops::{Deref, DerefMut};

pub struct Output<Field>(pub Field);
pub struct SizedOutput<Field>(pub Field, pub usize);

impl<Field> Deref for Output<Field> {
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Field> Deref for SizedOutput<Field> {
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Field> DerefMut for SizedOutput<Field> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
