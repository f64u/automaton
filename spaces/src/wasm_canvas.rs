use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, Index},
    space::{OutputField, Space},
    world::BasicWorld,
};
use wasm_bindgen::prelude::*;

use crate::common::OutputManager;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "setClass")]
    pub fn set_class(x: usize, y: usize, new_class: HtmlClass);
}

pub type HtmlClass = String;

pub type Classes = OutputManager<()>;

impl<C> OutputField<C, HtmlClass> for Classes
where
    C: BasicCell,
{
    fn set_unit(&mut self, (x, y): Index, unit: HtmlClass, _refresh: bool) -> Result<(), String> {
        set_class(x, y, unit); // let's see
        Ok(())
    }

    fn show(&mut self) {}
}

pub struct Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    world: W,
    output: Classes,
    reprer: fn(C) -> HtmlClass,
}

impl<'a, W, C> Space<W, C, Classes> for Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    type CellRepr = HtmlClass;
    type Reprer = fn(C) -> HtmlClass;
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Classes {
        &mut self.output
    }

    fn reprer(&self) -> Self::Reprer {
        self.reprer
    }
}

impl<W, C> Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    pub fn new(world: W, output: Classes, reprer: fn(C) -> HtmlClass) -> Self {
        Self {
            world,
            output,
            reprer,
        }
    }
}

pub fn build_web<W, C>(
    dimensions: Dimensions,
    repr: fn(C) -> HtmlClass,
    pixel_size: usize,
) -> Browser<W, C>
where
    W: BasicWorld<C>,
    C: BasicCell,
{
    let mut rng = rand::thread_rng();
    let world = W::random(&mut rng, dimensions);
    let output = Browser::new(
        world,
        OutputManager {
            field: (),
            pixel_size,
        },
        repr,
    );
    output
}
