use wasm_bindgen::prelude::*;

mod app;
mod game_loop;
mod input;
mod net;
mod renderer;
mod simulation;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    game_loop::start();
}
