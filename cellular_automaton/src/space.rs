use crate::{common::Representable, world::BasicWorld};

pub trait Space<W, O, T>
where
    W: BasicWorld + Representable<T>,
    O: OutputField<Data = T>,
{
    fn world_mut(&mut self) -> &mut W;
    fn world(&self) -> &W;
    fn output_mut(&mut self) -> &mut O;

    fn update(&mut self) -> Result<(), String> {
        self.world_mut().tick();
        let data = self.world().represent();
        self.output_mut().update(data)
    }
}

pub trait OutputField {
    type Data;
    fn update(&mut self, data: Self::Data) -> Result<(), String>;
}
