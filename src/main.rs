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
        "Julia Sets. (WASD to move, Arrow Keys to modify seed, R to reset)",
        WIDTH,
        HEIGHT,
        rules::julia,
    );
    app.main_loop();
}
