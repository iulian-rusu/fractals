#![feature(
    unboxed_closures,
    fn_traits,
    tuple_trait,
    int_roundings,
    const_option,
    portable_simd
)]

use crate::app::FractalExplorerApp;
use color::palettes;
use simd::SimdComplex;

mod app;
mod color;
mod render;
mod rules;
mod simd;
mod utils;
mod view;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut app = FractalExplorerApp::new(
        "Fractal Explorer. (WASD to move, Arrow Keys to modify seed, R to reset)",
        WIDTH,
        HEIGHT,
        |z, seed| {
            palettes::YELLOW_RED.color_array(rules::julia(z, SimdComplex::from_complex(seed)))
        },
    );
    app.main_loop();
}
