use crate::game::{Cell, World};
use cellular_automaton::{
    common::{Grid, Index, RepresentableAs},
    world::BasicWorld,
};
use spaces::wasm_canvas::{Html, WebCell, WebWorld};
use wasm_bindgen::prelude::*;

use std::{cell::RefCell, ops::Deref};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

impl RepresentableAs<Html> for Cell {
    type Delta = Html;
    fn represent(&self) -> Html {
        Html {
            value: format!(
                "<span class=\"{}\"></span>",
                match *self {
                    Cell::Alive => "alive",
                    Cell::Dead => "dead",
                }
            ),
        }
    }

    fn next_frame(&self) -> Self::Delta {
        self.represent()
    }
}

impl WebCell for Cell {}

impl<const W: usize, const H: usize> RepresentableAs<Grid<Html, W, H>> for World<W, H> {
    type Delta = Vec<(Index, Html)>;

    fn represent(&self) -> Grid<Html, W, H> {
        WebWorld::represent(self)
    }

    fn next_frame(&self) -> Self::Delta {
        self.delta_future()
            .into_iter()
            .map(|(index, cell)| (index, RepresentableAs::<Html>::represent(&cell)))
            .collect()
    }
}

impl<const W: usize, const H: usize> WebWorld<W, H> for World<W, H> {}

#[wasm_bindgen(js_name = "getState")]
pub fn get_state() -> String {
    WORLD.with(|w| {
        let world = w.borrow();
        WebWorld::represent(world.deref());
        String::new()
    })
}

#[wasm_bindgen]
pub fn tick() {
    WORLD.with(|w| w.borrow_mut().tick())
}

thread_local! {
    static WORLD: RefCell<World<120, 70>> = RefCell::new(World::new_random());
}
