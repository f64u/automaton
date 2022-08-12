use std::cell::RefCell;

use auto_cellular::{common::Dimensions, space::Space};
use auto_spaces::wasm_canvas::{build_web, Browser};

use auto_worlds::briansbrain::{Cell as BrainCell, World as BrainWorld};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
}

const CONFIG: Config = Config {
    width: 700,
    height: 700,
    pixel_size: 10,
};

#[wasm_bindgen(js_name = "getConfig")]
pub fn get_config() -> Config {
    CONFIG
}

#[wasm_bindgen(js_name = "worldClick")]
pub fn world_click(x: usize, y: usize) {
    BROWSER.with(|b| {
        let mut b = b.borrow_mut();
        b.click_world((x, y));
        let _ = b.draw_delta();
    })
}

#[wasm_bindgen(js_name = "blankWorld")]
pub fn blank_world() {
    BROWSER.with(|b| {
        let mut b = b.borrow_mut();
        let _ = b.replace_with_blank_world();
    })
}

#[wasm_bindgen(js_name = "worldReload")]
pub fn world_reload() {
    BROWSER.with(|b| {
        let mut b = b.borrow_mut();
        let _ = b.replace_with_random_world();
    })
}

#[wasm_bindgen(js_name = "tickBriansWorld")]
pub fn tick_brians_world() {
    BROWSER.with(|b| {
        let mut b = b.borrow_mut();
        let _ = b.tick_delta();
    })
}

#[wasm_bindgen(js_name = "firstDrawBrains")]
pub fn first_draw_brains() {
    BROWSER.with(|b| {
        let mut b = b.borrow_mut();
        let _ = b.draw_whole();
    })
}

thread_local! {
  static BROWSER: RefCell<Browser<BrainWorld>> = RefCell::new(build_web(Dimensions(CONFIG.width / CONFIG.pixel_size, CONFIG.height / CONFIG.pixel_size), |c| match c {
      BrainCell::On => "white",
      BrainCell::Dying => "blue",
      BrainCell::Off => "black"
  }.into(), CONFIG.pixel_size));
}
