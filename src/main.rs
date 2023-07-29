#![feature(unboxed_closures, fn_traits, tuple_trait)]

use crate::app::FractalExplorerApp;

mod app;
mod color;
mod render;
mod rules;
mod shared;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let mut app = FractalExplorerApp::new(
        "Fractal Explorer. (WASD to move, Arrow Keys to modify seed, R to reset)",
        WIDTH,
        HEIGHT,
        |z, seed| color::grayscale(rules::julia(z, seed)),
    );
    app.main_loop();
}
