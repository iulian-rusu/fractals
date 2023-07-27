use futures::executor::ThreadPool;
use minifb::{Key, Window, WindowOptions};

use crate::{
    color::color_grayscale,
    render::{Pixel, RenderConfig, render_pixels},
    rules::julia,
    shared::{directon, Complex},
};

const INITIAL_SCALE: f64 = 1.0;
const ZOOM_FACTOR: f64 = 0.85;
const SEED_DELTA: f64 = 0.001;
const OFFSET_DELTA: f64 = 0.02;
const RENDER_THREADS: usize = 14;
const INITIAL_SEED: Complex = Complex::new(-0.78, 0.136);

pub struct App {
    window: Window,
    buffer: Vec<u32>,
    thread_pool: ThreadPool,
    width: usize,
    height: usize,
    offset: Complex,
    seed: Complex,
    scale: f64,
    should_update: bool,
}

impl App {
    pub fn new(title: impl AsRef<str>, width: usize, height: usize) -> Self {
        Self {
            window: Window::new(title.as_ref(), width, height, WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e)),
            buffer: vec![0u32; (width * height) as usize],
            thread_pool: ThreadPool::builder()
                .pool_size(RENDER_THREADS)
                .create()
                .expect("Failed to create thread pool"),
            width,
            height,
            offset: Complex::default(),
            seed: INITIAL_SEED,
            scale: INITIAL_SCALE,
            should_update: true,
        }
    }

    pub fn main_loop(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
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

            self.update();

            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap_or_else(|e| println!("Error updating window with buffer: {}", e));
        }
    }

    fn update_scale(&mut self, scroll_y: f32) {
        if scroll_y > 0f32 {
            self.scale *= ZOOM_FACTOR;
        } else {
            self.scale /= ZOOM_FACTOR;
        }
        self.should_update = true;
    }

    fn update_offset(&mut self, direction: Complex) {
        self.offset += direction * OFFSET_DELTA * self.scale;
        self.should_update = true;
    }

    fn update_seed(&mut self, direction: Complex) {
        self.seed += direction * SEED_DELTA * self.scale;
        self.should_update = true;
    }

    fn reset(&mut self) {
        self.offset = Complex::default();
        self.seed = INITIAL_SEED;
        self.scale = INITIAL_SCALE;
        self.should_update = true;
    }

    fn update(&mut self) {
        if !self.should_update {
            return;
        }

        let pixels = render_pixels(
            RenderConfig {
                scale: self.scale,
                width: self.width,
                height: self.height,
                thread_pool: &self.thread_pool,
                pool_size: RENDER_THREADS,
            },
            {
                let seed = self.seed;
                let offset = self.offset;
                move |z| color_grayscale(julia(seed, z - offset))
            },
        );

        self.buffer = pixels
            .into_iter()
            .map(|Pixel { color, .. }| u32::from_be_bytes([0, color.0, color.1, color.2]))
            .collect();

        self.print_state();
        self.should_update = false;
    }

    pub fn print_state(&self) {
        println!("Offset = {}", self.offset);
        println!("Seed   = {}", self.seed);
        println!("Scale  = {:+e}\n\n\n", self.scale);
    }
}
