use crate::{
    color::Rgb,
    render::Renderer,
    shared::{ColorComputer, Complex, Direction},
    view::ComplexPlaneView,
};
use minifb::{Key, Window, WindowOptions};
use std::{
    num::NonZeroUsize,
    thread,
    time::{Duration, Instant},
};

pub struct FractalExplorerApp<F: ColorComputer(Complex, Complex) -> Rgb> {
    window: Window,
    renderer: Renderer,
    buffer: Vec<u32>,
    view: ComplexPlaneView,
    color_computer: F,
    seed: Complex,
    should_redraw: bool,
}

impl<F: ColorComputer(Complex, Complex) -> Rgb> FractalExplorerApp<F> {
    const INITIAL_SEED: Complex = Complex::new(-0.7768, 0.1374);
    const BASE_SEED_STEP: f64 = 0.001;
    const DEFAULT_RENDER_THREAD_COUNT: usize = 16;
    const FRAMES_PER_SECOND: u32 = 60;
    const FRAME_DURATION: Duration = Duration::from_secs(1)
        .checked_div(Self::FRAMES_PER_SECOND)
        .expect("FPS should not be zero");

    pub fn new(title: impl AsRef<str>, width: usize, height: usize, color_computer: F) -> Self {
        Self {
            window: Window::new(title.as_ref(), width, height, WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e)),
            renderer: Renderer::new(Self::resolve_render_thread_count()),
            buffer: vec![0u32; width * height],
            view: ComplexPlaneView::new(width, height),
            color_computer,
            seed: Self::INITIAL_SEED,
            should_redraw: true,
        }
    }

    fn resolve_render_thread_count() -> usize {
        std::thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(Self::DEFAULT_RENDER_THREAD_COUNT)
    }

    pub fn main_loop(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let start = Instant::now();
            self.update();
            Self::delay_until_next_frame(start.elapsed());
        }
    }

    fn delay_until_next_frame(elapsed: Duration) {
        if elapsed < Self::FRAME_DURATION {
            thread::sleep(Self::FRAME_DURATION - elapsed);
        }
    }

    fn update(&mut self) {
        if let Some((_, y)) = self.window.get_scroll_wheel() {
            self.update_view_scale(y);
        }

        self.window.get_keys().iter().for_each(|&k| match k {
            Key::W => self.translate_view(Direction::Up),
            Key::S => self.translate_view(Direction::Down),
            Key::A => self.translate_view(Direction::Left),
            Key::D => self.translate_view(Direction::Right),
            Key::Up => self.translate_seed(Direction::Up),
            Key::Down => self.translate_seed(Direction::Down),
            Key::Left => self.translate_seed(Direction::Left),
            Key::Right => self.translate_seed(Direction::Right),
            Key::R => self.reset(),
            _ => (),
        });

        if self.should_redraw {
            self.redraw();
        }
        self.window
            .update_with_buffer(&self.buffer, self.view.width(), self.view.height())
            .unwrap_or_else(|e| println!("Error updating window with buffer: {}", e));
    }

    fn update_view_scale(&mut self, scroll_y: f32) {
        if scroll_y > 0.0 {
            self.view.zoom_out()
        } else {
            self.view.zoom_in()
        }
        self.should_redraw = true;
    }

    fn translate_view(&mut self, direction: Direction) {
        self.view.translate(direction);
        self.should_redraw = true;
    }

    fn translate_seed(&mut self, direction: Direction) {
        self.seed += direction.as_complex() * Self::BASE_SEED_STEP * self.view.scale();
        self.should_redraw = true;
    }

    fn reset(&mut self) {
        self.view.reset();
        self.seed = Self::INITIAL_SEED;
        self.should_redraw = true;
    }

    fn redraw(&mut self) {
        let start = Instant::now();
        let pixels = self.renderer.render(&self.view, {
            let seed = self.seed;
            let color_computer = self.color_computer.clone();
            move |z| color_computer(z, seed)
        });
        self.buffer.clear();
        self.buffer.extend(pixels.map(Rgb::as_u32));
        self.should_redraw = false;
        self.display_metrics(start.elapsed());
    }

    fn display_metrics(&self, elapsed: Duration) {
        println!(
            "[Rendered {}px ({}x{}) in {}ms ({} FPS)]",
            self.buffer.len(),
            self.view.width(),
            self.view.height(),
            elapsed.as_millis(),
            1000 / elapsed.as_millis()
        );
        println!("Scale = {:+e}", self.view.scale());
        println!("ViewOffset = {}", self.view.offset());
        println!("Seed = {}", self.seed);
        println!("\n\n");
    }
}
