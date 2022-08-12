use auto_cellular::{
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

pub struct Browser<W>
where
    W: BasicWorld,
{
    world: W,
    output: Colors,
    reprer: fn(W::Cell) -> Color,
}

impl<'a, W> Space<W, Colors> for Browser<W>
where
    W: BasicWorld,
{
    type CellRepr = Color;
    type Reprer = fn(W::Cell) -> Color;
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

impl<W> Browser<W>
where
    W: BasicWorld,
{
    pub fn new(world: W, output: Colors, reprer: fn(W::Cell) -> Color) -> Self {
        Self {
            world,
            output,
            reprer,
        }
    }
}

pub fn build_web<W>(
    dimensions: Dimensions,
    repr: fn(W::Cell) -> Color,
    pixel_size: usize,
) -> Browser<W>
where
    W: BasicWorld,
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
