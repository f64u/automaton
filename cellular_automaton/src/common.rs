pub trait Representable<As> {
    fn represent(&self) -> As;
}

pub type Position = (isize, isize);

#[derive(Clone, Copy)]
pub struct Dimensions(pub usize, pub usize);

impl Dimensions {
    pub fn get_index(&self, (x, y): Position) -> Option<usize> {
        if x < 0 || y < 0 || x as usize > self.0 || y as usize > self.1 {
            None
        } else {
            Some(self.0 * y as usize + x as usize)
        }
    }

    pub fn get_pos(&self, index: usize) -> Position {
        ((index % self.0) as isize, (index / self.0) as isize)
    }
}
