use std::{num::NonZeroUsize, time::Instant};

use minifb::{Key, Window, WindowOptions};

use crate::{
    color::Rgb,
    render::{RenderParams, Renderer},
    shared::{directon, ColorComputer, Complex},
};

const INITIAL_SCALE: f64 = 1.0;
const ZOOM_FACTOR: f64 = 0.85;
const SEED_DELTA: f64 = 0.001;
const OFFSET_DELTA: f64 = 0.02;
const INITIAL_OFFSET: Complex = Complex::new(0.0, 0.0);
const INITIAL_SEED: Complex = Complex::new(-0.78, 0.136);
const DEFAULT_RENDER_THREAD_COUNT: usize = 16;

pub struct FractalExplorerApp<F: ColorComputer(Complex, Complex) -> Rgb> {
    window: Window,
    renderer: Renderer,
    buffer: Vec<u32>,
    color_computer: F,
    width: usize,
    height: usize,
    scale: f64,
    offset: Complex,
    seed: Complex,
    should_redraw: bool,
}

fn resolve_render_thread_count() -> usize {
    std::thread::available_parallelism()
        .map(NonZeroUsize::get)
        .unwrap_or(DEFAULT_RENDER_THREAD_COUNT)
}

impl<F: ColorComputer(Complex, Complex) -> Rgb> FractalExplorerApp<F> {
    pub fn new(title: impl AsRef<str>, width: usize, height: usize, color_computer: F) -> Self {
        Self {
            window: Window::new(title.as_ref(), width, height, WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e)),
            renderer: Renderer::new(resolve_render_thread_count()),
            buffer: vec![0u32; width * height],
            color_computer,
            width,
            height,
            scale: INITIAL_SCALE,
            offset: INITIAL_OFFSET,
            seed: INITIAL_SEED,
            should_redraw: true,
        }
    }

    pub fn main_loop(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();
        }
    }

    fn update(&mut self) {
        if let Some((_, y)) = self.window.get_scroll_wheel() {
            self.update_scale(y);
        }

        self.window.get_keys().iter().for_each(|&k| match k {
            Key::W => self.update_offset(directon::UP),
            Key::S => self.update_offset(directon::DOWN),
            Key::A => self.update_offset(directon::LEFT),
            Key::D => self.update_offset(directon::RIGHT),
            Key::Up => self.update_seed(directon::UP),
            Key::Down => self.update_seed(directon::DOWN),
            Key::Left => self.update_seed(directon::LEFT),
            Key::Right => self.update_seed(directon::RIGHT),
            Key::R => self.reset(),
            _ => (),
        });

        if self.should_redraw {
            self.redraw();
        }

        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap_or_else(|e| println!("Error updating window with buffer: {}", e));
    }

    fn update_scale(&mut self, scroll_y: f32) {
        if scroll_y > 0f32 {
            self.scale *= ZOOM_FACTOR;
        } else {
            self.scale /= ZOOM_FACTOR;
        }
        self.should_redraw = true;
    }

    fn update_offset(&mut self, direction: Complex) {
        self.offset += direction * OFFSET_DELTA * self.scale;
        self.should_redraw = true;
    }

    fn update_seed(&mut self, direction: Complex) {
        self.seed += direction * SEED_DELTA * self.scale;
        self.should_redraw = true;
    }

    fn reset(&mut self) {
        self.scale = INITIAL_SCALE;
        self.offset = INITIAL_OFFSET;
        self.seed = INITIAL_SEED;
        self.should_redraw = true;
    }

    fn redraw(&mut self) {
        let start = Instant::now();
        let pixels = self.renderer.render_pixels(RenderParams {
            width: self.width,
            height: self.height,
            scale: self.scale,
            offset: self.offset,
            color_computer: {
                let seed = self.seed;
                let color_computer = self.color_computer.clone();
                move |z| color_computer(z, seed)
            },
        });
        self.buffer = pixels
            .into_iter()
            .map(|color| u32::from_be_bytes([0, color.0, color.1, color.2]))
            .collect();
        let elapsed = start.elapsed();

        println!("[Rendered {} pixels in {} ms]", self.buffer.len(), elapsed.as_millis());
        self.print_state();
        self.should_redraw = false;
    }

    pub fn print_state(&self) {
        println!("Scale  = {:+e}", self.scale);
        println!("Offset = {}", self.offset);
        println!("Seed   = {}\n\n\n", self.seed);
    }
}
