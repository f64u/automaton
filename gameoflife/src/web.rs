use crate::game::{Cell, World};
use cellular_automaton::{
    common::{Dimensions, Representable},
    world::BasicWorld,
};
use spaces::wasm_canvas::{Html, WebCell, WebWorld};
use wasm_bindgen::prelude::*;

use std::{cell::RefCell, ops::Deref};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

impl Representable<Html> for Cell {
    fn represent(&self) -> Html {
        Html {
            value: if self.is_alive() {
                "<span class=\"alive\"></span>"
            } else {
                "<span class=\"dead\"></span>"
            }
            .into(),
        }
    }
}

impl WebCell for Cell {}

impl Representable<Html> for World {
    fn represent(&self) -> Html {
        WebWorld::represent(self)
    }
}

impl WebWorld for World {}

#[wasm_bindgen(js_name = "getState")]
pub fn get_state() -> String {
    WORLD.with(|w| {
        let world = w.borrow();
        WebWorld::represent(world.deref()).to_string()
    })
}

#[wasm_bindgen]
pub fn tick() {
    WORLD.with(|w| w.borrow_mut().tick())
}

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::new_random(Dimensions(120, 70)));
}
