use crate::{
    cell::BasicCell,
    common::{DoubleVec, Index},
    world::BasicWorld,
};

/// A [Space] (like a Gui) where a [BasicWorld] can show [BasicCell]s
/// It only cares about learning how to lay the cells in the world

pub trait Space<W, C, O>
where
    C: BasicCell,
    W: BasicWorld<C>,
    O: OutputField<C, Self::CellRepr>,
{
    type CellRepr;

    fn world_mut(&mut self) -> &mut W;
    fn world(&self) -> &W;
    fn output_mut(&mut self) -> &mut O;

    fn draw_whole<F>(&mut self, repr: F) -> Result<(), String>
    where
        F: Fn(C) -> Self::CellRepr,
    {
        let data = self
            .world()
            .cells()
            .iter()
            .map(|row| row.iter().map(|cell| repr(*cell)).collect())
            .collect();
        self.output_mut().set_all(data)
    }

    fn draw_delta<F>(&mut self, repr: F) -> Result<(), String>
    where
        F: Fn(C) -> Self::CellRepr,
    {
        let changes = self.world().delta().clone();
        let next = changes.into_iter().map(|(index, cell)| (index, repr(cell)));
        self.output_mut().update(next)
    }

    fn tick_whole<F>(&mut self, repr: F) -> Result<(), String>
    where
        F: Fn(C) -> Self::CellRepr,
    {
        self.world_mut().tick();
        self.draw_whole(repr)
    }

    fn tick_delta<F>(&mut self, repr: F) -> Result<(), String>
    where
        F: Fn(C) -> Self::CellRepr,
    {
        self.world_mut().tick();
        self.draw_delta(repr)
    }
}

pub trait OutputField<C, S>
where
    C: BasicCell,
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
