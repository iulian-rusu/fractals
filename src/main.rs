#![feature(unboxed_closures, fn_traits, tuple_trait, int_roundings, const_option)]

use crate::app::FractalExplorerApp;
use color::palettes;

mod app;
mod color;
mod render;
mod rules;
mod shared;
mod viewport;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut app = FractalExplorerApp::new(
        "Fractal Explorer. (WASD to move, Arrow Keys to modify seed, R to reset)",
        WIDTH,
        HEIGHT,
        |z, seed| palettes::BLUE_GREEN.color(rules::julia(z, seed)),
    );
    app.main_loop();
}
