use crate::game::{Cell, World};
use cellular_automaton::{
    common::{Dimensions, Index, Repr},
    world::BasicWorld,
};
use spaces::wasm_canvas::Html;
use wasm_bindgen::prelude::*;

use std::cell::RefCell;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

impl Repr<Html> for Cell {
    fn repr(&self) -> Html {
        todo!()
    }
}

#[wasm_bindgen(js_name = "getState")]
pub fn get_state() -> String {
    WORLD.with(|w| {
        let world = w.borrow();
        // WebWorld::represent(world.deref());
        String::new()
    })
}

#[wasm_bindgen]
pub fn tick() {
    WORLD.with(|w| w.borrow_mut().tick())
}

thread_local! {
    static WORLD: RefCell<World<Cell>> = RefCell::new(World::random(&mut rand::thread_rng(), Dimensions(120, 70)));
}
