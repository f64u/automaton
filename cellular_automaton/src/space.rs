use crate::{
    cell::BasicCell,
    common::{DoubleVec, Index, Repr},
    world::BasicWorld,
};

/// A [Space] (like a Gui) where a [BasicWorld] can show [BasicCell]s
/// It only cares about learning how to lay the cells in the world

pub trait Space<W, C, O>
where
    C: BasicCell + Repr<Self::CellRepr>,
    W: BasicWorld<C>,
    O: OutputField<C, Self::CellRepr>,
{
    type CellRepr;

    fn world_mut(&mut self) -> &mut W;
    fn world(&self) -> &W;
    fn output_mut(&mut self) -> &mut O;

    fn draw_whole(&mut self) -> Result<(), String> {
        let data = self
            .world()
            .cells()
            .iter()
            .map(|row| row.iter().map(|cell| cell.repr()).collect())
            .collect();
        self.output_mut().set_all(data)
    }

    fn draw_delta(&mut self) -> Result<(), String> {
        let changes = self.world().delta().clone();
        let next = changes
            .into_iter()
            .map(|(index, cell)| (index, cell.repr()));
        self.output_mut().update(next)
    }

    fn tick_whole(&mut self) -> Result<(), String> {
        self.world_mut().tick();
        self.draw_whole()
    }

    fn tick_delta(&mut self) -> Result<(), String> {
        self.world_mut().tick();
        self.draw_delta()
    }
}

pub trait OutputField<C, S>
where
    C: BasicCell + Repr<S>,
{
    fn set_unit(&mut self, index: Index, unit: S, refresh: bool) -> Result<(), String>;
    fn show(&mut self);

    fn set_all(&mut self, data: DoubleVec<S>) -> Result<(), String> {
        for (y, row) in data.into_iter().enumerate() {
            for (x, unit) in row.into_iter().enumerate() {
                self.set_unit((x, y), unit, false)?
            }
        }
        self.show();
        Ok(())
    }

    fn update(&mut self, delta: impl Iterator<Item = (Index, S)>) -> Result<(), String> {
        for (index, cell) in delta {
            self.set_unit(index, cell, false)?
        }
        self.show();
        Ok(())
    }
}
