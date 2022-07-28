use std::cell::RefCell;

use cellular_automaton::{common::Dimensions, space::Space};
use spaces::wasm_canvas::{build_web, Browser};

use briansbrain::{cell::Cell as BrainCell, world::World as BrainWorld};
// use gameoflife::{cell::Cell as LifeCell, world::World as LifeWorld};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Dim(pub usize, pub usize);

const DIMENSIONS: Dimensions = Dimensions(120, 70);

#[wasm_bindgen(js_name = "getDimensions")]
pub fn get_dimensions() -> Dim {
    Dim(DIMENSIONS.0, DIMENSIONS.1)
}

#[wasm_bindgen(js_name = "tickBriansWorld")]
pub fn tick_brians_world() {
    BROWSER.with(|b| {
        let mut b = b.borrow_mut();
        let _ = b.tick_delta();
    })
}

// #[wasm_bindgen(js_name = "firstDrawBrains")]
// pub fn first_draw_brains() {
//     BROWSER.with(|b| {
//         let mut b = b.borrow_mut();
//         let _ = b.draw_whole();
//     })
// }

thread_local! {
  static BROWSER: RefCell<Browser<BrainWorld, BrainCell>> = RefCell::new(build_web(DIMENSIONS, |c| match c {
      BrainCell::On => "on",
      BrainCell::Dying => "dying",
      BrainCell::Off => "off"
  }.into(), 1));
}
