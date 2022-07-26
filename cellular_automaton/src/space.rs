use crate::{
    common::{Index, RepresentableAs},
    world::BasicWorld,
};

pub trait Space<World, O, U, const W: usize, const H: usize>
where
    World: BasicWorld<W, H> + RepresentableAs<[[U; W]; H], Delta = Vec<(Index, U)>>,
    O: OutputField<W, H, Unit = U>,
{
    fn world_mut(&mut self) -> &mut World;
    fn world(&self) -> &World;
    fn output_mut(&mut self) -> &mut O;

    fn draw_whole_world(&mut self) -> Result<(), String> {
        let data = self.world().represent();
        self.output_mut().set_all(data)
    }

    fn draw_delta(&mut self) -> Result<(), String> {
        let next = RepresentableAs::next_frame(self.world());
        self.output_mut().update(next.into_iter())
    }

    fn tick(&mut self) -> Result<(), String> {
        self.draw_delta().map(|_| self.world_mut().tick())
    }
}

pub trait OutputField<const W: usize, const H: usize> {
    type Unit;

    fn set_unit(&mut self, index: Index, unit: Self::Unit, refresh: bool) -> Result<(), String>;
    fn show(&mut self);

    fn set_all(&mut self, data: [[Self::Unit; W]; H]) -> Result<(), String> {
        for (y, row) in data.into_iter().enumerate() {
            for (x, unit) in row.into_iter().enumerate() {
                self.set_unit((x, y), unit, false)?
            }
        }
        self.show();
        Ok(())
    }

    fn update(&mut self, delta: impl Iterator<Item = (Index, Self::Unit)>) -> Result<(), String> {
        for (index, cell) in delta {
            self.set_unit(index, cell, false)?
        }
        self.show();
        Ok(())
    }
}
