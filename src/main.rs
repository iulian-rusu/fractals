#![feature(unboxed_closures, tuple_trait, const_option, portable_simd)]

use crate::{app::FractalExplorerApp, color::palettes, simd::SimdComplex};

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
            palettes::BLUE_GREEN.color_array(rules::julia(z, SimdComplex::from_complex(seed)))
        },
    );
    app.main_loop();
}
