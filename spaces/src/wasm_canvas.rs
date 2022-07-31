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
    #[wasm_bindgen(js_name = "setPixel")]
    pub fn set_pixel(x: usize, y: usize, pixel_size: usize, new_class: Color);
}

pub type Color = String;

pub type Colors = OutputManager<()>;

impl<C> OutputField<C, Color> for Colors
where
    C: BasicCell,
{
    fn set_unit(&mut self, (x, y): Index, unit: Color, _refresh: bool) -> Result<(), String> {
        set_pixel(x, y, self.pixel_size, unit); // let's see
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
    output: Colors,
    reprer: fn(C) -> Color,
}

impl<'a, W, C> Space<W, C, Colors> for Browser<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    type CellRepr = Color;
    type Reprer = fn(C) -> Color;
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Colors {
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
    pub fn new(world: W, output: Colors, reprer: fn(C) -> Color) -> Self {
        Self {
            world,
            output,
            reprer,
        }
    }
}

pub fn build_web<W, C>(
    dimensions: Dimensions,
    repr: fn(C) -> Color,
    pixel_size: usize,
) -> Browser<W, C>
where
    W: BasicWorld<C>,
    C: BasicCell,
{
    let mut rng = rand::thread_rng();
    let world = W::new_random(&mut rng, dimensions);
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
