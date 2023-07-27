use crate::app::App;
mod app;
mod color;
mod render;
mod rules;
mod shared;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let mut app = App::new(
        "Julia Sets. (WASD to move, Arrow Keys to modify seed, R to reset)",
        WIDTH,
        HEIGHT,
    );
    app.main_loop();
}
