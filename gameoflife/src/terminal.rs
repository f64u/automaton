use cellular_automaton::{
    common::{Grid, Index, RepresentableAs},
    world::BasicWorld,
};
use spaces::cursive_canvas::{self, CharCell, StringWorld};

use crate::game::{Cell, World};

impl RepresentableAs<char> for Cell {
    type Delta = char;
    fn represent(&self) -> char {
        match *self {
            Cell::Alive => '#',
            Cell::Dead => ' ',
        }
    }
    fn next_frame(&self) -> Self::Delta {
        self.represent()
    }
}

impl CharCell for Cell {}

impl<const W: usize, const H: usize> RepresentableAs<Grid<char, W, H>> for World<W, H> {
    type Delta = Vec<(Index, char)>;
    fn represent(&self) -> Grid<char, W, H> {
        StringWorld::represent(self)
    }
    fn next_frame(&self) -> Self::Delta {
        self.delta_future()
            .into_iter()
            .map(|(index, cell)| (index, RepresentableAs::<char>::represent(&cell)))
            .collect()
    }
}

impl<const W: usize, const H: usize> StringWorld<W, H> for World<W, H> {}

pub(crate) fn run() -> Result<(), String> {
    const W: usize = 70;
    const H: usize = 70;
    let world = World::<W, H>::new_random();
    cursive_canvas::run(world)?;

    Ok(())
}
