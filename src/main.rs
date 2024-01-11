#![feature(unboxed_closures, fn_traits, tuple_trait, int_roundings, const_option)]
use crate::app::FractalExplorerApp;
use color::palettes;

mod app;
mod color;
mod render;
mod rules;
mod shared;
mod view;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut app = FractalExplorerApp::new(
        "Fractal Explorer. (WASD to move, Arrow Keys to modify seed, R to reset)",
        WIDTH,
        HEIGHT,
        |z, seed| {
            palettes::BLUE_GREEN.color(rules::nova(
                z,
                seed,
                |z| (z - 1.0).powu(3),
                |z| 2.0 * z.powu(2),
            ))
        },
    );
    app.main_loop();
}
