use crate::{
    cell::BasicCell,
    common::{DoubleVec, Index},
    world::BasicWorld,
};

/// A [Space] (like a Gui) where a [BasicWorld] can show its [BasicCell]s
/// It only cares about learning how to lay the cells in the world
pub trait Space<W, O>
where
    W: BasicWorld,
    O: OutputField<W::Cell, Self::CellRepr>,
{
    /// The [Space]'s reprsentation of the [BasicCell]s in the [OutputField]
    type CellRepr;
    /// A pure [Fn] that takes a [BasicCell] and returns that its representation
    type Reprer: Fn(W::Cell) -> Self::CellRepr;

    /// Get a mutable reference to the [BasicWorld] the [Space] manages
    fn world_mut(&mut self) -> &mut W;

    /// Get a shared reference to the [BasicWorld] the [Space] manages
    fn world(&self) -> &W;

    /// Get a mutable referene to the [OutputField] of this [Space]
    fn output_mut(&mut self) -> &mut O;

    /// Get the representer of the [Space]
    fn reprer(&self) -> Self::Reprer;

    /// (Re)draw the whole [BasicWorld] in the [OutputField]
    fn draw_whole(&mut self) -> Result<(), String> {
        let data = self
            .world()
            .cells()
            .iter()
            .map(|row| row.iter().map(|cell| self.reprer()(*cell)).collect())
            .collect();
        self.output_mut().set_all(data)
    }

    /// Draw only the changes that the [BasicWorld] experienced the previous tick
    fn draw_delta(&mut self) -> Result<(), String> {
        let changes = self.world().delta().clone();
        let repr = self.reprer();
        let next = changes.into_iter().map(|(index, cell)| (index, repr(cell)));
        self.output_mut().update(next)
    }

    /// One tick passes in the [BasicWorld] and the whole [BasicWorld] is redrawn
    fn tick_whole(&mut self) -> Result<(), String> {
        self.world_mut().tick();
        self.draw_whole()
    }

    /// One tick passes in the [BasicWorld] and only the deltas are redrawn
    fn tick_delta(&mut self) -> Result<(), String> {
        self.world_mut().tick();
        self.draw_delta()
    }

    /// Replace the [Space]'s [BasicWorld] with a random one
    fn replace_with_random_world(&mut self) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        *self.world_mut() = self.world().random(&mut rng);
        self.draw_whole()
    }

    /// Replace the [Space]'s [BasicWorld] with a blank wone
    fn replace_with_blank_world(&mut self) -> Result<(), String> {
        *self.world_mut() = self.world().blank();
        self.draw_whole()
    }

    /// Propagate a click that happened on the [BasicWorld] at the [Index] (x, y)
    fn click_world(&mut self, i: Index) {
        self.world_mut().click(i);
    }
}

/// An abstraction over the pixels of a gui or the character space of a terminal and so on
pub trait OutputField<C, S>
where
    C: BasicCell,
{
    /// Sets one unit (e.g. a pixel) to the S, which is the corresponding representation
    /// of a [BasicCell] in a given [Space]
    fn set_unit(&mut self, index: Index, unit: S, refresh: bool) -> Result<(), String>;

    /// If the output field is a buffer, commit the changes to memeory
    fn show(&mut self);

    /// Updates the whole output field with new representations of [BasicCell]s
    fn set_all(&mut self, data: DoubleVec<S>) -> Result<(), String> {
        for (y, row) in data.into_iter().enumerate() {
            for (x, unit) in row.into_iter().enumerate() {
                self.set_unit((x, y), unit, false)?
            }
        }
        self.show();
        Ok(())
    }

    /// Only changes the delta
    fn update(&mut self, delta: impl Iterator<Item = (Index, S)>) -> Result<(), String> {
        for (index, cell) in delta {
            self.set_unit(index, cell, false)?
        }
        self.show();
        Ok(())
    }
}
