use crate::{common::Representable, world::BasicWorld};

pub trait Space<W, O, T>
where
    W: BasicWorld + Representable<T>,
    O: OutputField<Data = T>,
{
    fn world_mut(&mut self) -> &mut W;
    fn world(&self) -> &W;
    fn output_mut(&mut self) -> &mut O;

    fn draw(&mut self) -> Result<(), String> {
        let data = self.world().represent();
        self.output_mut().update(data)
    }

    fn tick(&mut self) {
        self.world_mut().tick();
    }

    fn update(&mut self) -> Result<(), String> {
        self.tick();
        self.draw()
    }
}

pub trait OutputField {
    type Data;
    fn update(&mut self, data: Self::Data) -> Result<(), String>;
}
