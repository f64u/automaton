use std::marker::PhantomData;

use cellular_automaton::{
    cell::BasicCell,
    common::{DoubleVec, Index},
    space::{OutputField, Space},
    world::BasicWorld,
};

use crate::common::OutputManager;

pub struct Html {
    pub value: String,
}

impl ToString for Html {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

type Out = OutputManager<DoubleVec<Html>>;

impl<C> OutputField<C, Html> for Out
where
    C: BasicCell,
{
    fn set_unit(&mut self, (x, y): Index, unit: Html, _refresh: bool) -> Result<(), String> {
        self.field[y][x] = unit;
        Ok(())
    }

    fn show(&mut self) {}
}

struct Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    world: W,
    output: Out,
    _data: PhantomData<C>,
}

impl<'a, W, C> Space<W, C, Out> for Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    type CellRepr = Html;
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Out {
        &mut self.output
    }
}

impl<W, C> Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    fn new(world: W, output: Out) -> Self {
        Self {
            world,
            output,
            _data: PhantomData,
        }
    }
}
