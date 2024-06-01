use crate::{
    color::Rgb,
    render::Renderer,
    simd::{Array, SimdComplex},
    utils::{Complex, Direction, FnSync},
    view::ComplexPlaneView,
};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use minifb_fonts::{font6x8, FbFontRenderer};
use std::{
    thread,
    time::{Duration, Instant},
};

pub struct FractalExplorerApp<F>
where
    F: FnSync(SimdComplex, Complex) -> Array<Rgb>,
{
    window: Window,
    frame_renderer: Renderer,
    font_renderer: FbFontRenderer,
    frame_buffer: Vec<u32>,
    view: ComplexPlaneView,
    color_computer: F,
    seed: Complex,
    should_render: bool,
    display_stats: bool,
}

impl<F> FractalExplorerApp<F>
where
    F: FnSync(SimdComplex, Complex) -> Array<Rgb>,
{
    const INITIAL_SEED: Complex = Complex::new(-0.75, 0.2);
    const BASE_SEED_STEP: f64 = 0.001;
    const FONT_COLOR: Rgb = Rgb(255, 255, 255);
    const TEXT_POS_X: usize = 20;
    const FRAMES_PER_SECOND: u32 = 60;
    const FRAME_DURATION: Duration = Duration::from_secs(1)
        .checked_div(Self::FRAMES_PER_SECOND)
        .expect("FPS should not be zero");

    pub fn new(title: impl AsRef<str>, width: usize, height: usize, color_computer: F) -> Self {
        Self {
            window: Window::new(title.as_ref(), width, height, WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e)),
            frame_renderer: Renderer::new(),
            font_renderer: font6x8::new_renderer(width, height, Self::FONT_COLOR.as_u32()),
            frame_buffer: vec![0u32; width * height],
            view: ComplexPlaneView::new(width, height),
            color_computer,
            seed: Self::INITIAL_SEED,
            should_render: true,
            display_stats: false,
        }
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
            _ => (),
        });

        self.window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|&k| match k {
                Key::Q => self.toggle_stat_display(),
                Key::R => self.reset(),
                _ => (),
            });

        if self.should_render {
            self.render();
        }

        self.window
            .update_with_buffer(&self.frame_buffer, self.view.width(), self.view.height())
            .unwrap_or_else(|e| println!("Error updating main window with buffer: {}", e));
    }

    fn update_view_scale(&mut self, scroll_y: f32) {
        if scroll_y > 0.0 {
            self.view.zoom_out()
        } else {
            self.view.zoom_in()
        }
        self.should_render = true;
    }

    fn translate_view(&mut self, direction: Direction) {
        self.view.translate(direction);
        self.should_render = true;
    }

    fn translate_seed(&mut self, direction: Direction) {
        self.seed += direction.as_complex() * Self::BASE_SEED_STEP * self.view.scale();
        self.should_render = true;
    }

    fn toggle_stat_display(&mut self) {
        self.display_stats = !self.display_stats;
        self.should_render = true;
    }

    fn reset(&mut self) {
        self.view.reset();
        self.seed = Self::INITIAL_SEED;
        self.should_render = true;
    }

    fn render(&mut self) {
        let start = Instant::now();
        let pixels = self.frame_renderer.render(&self.view, {
            let seed = self.seed;
            let color_computer = &self.color_computer;
            move |z| color_computer(z, seed)
        });
        self.frame_buffer.clear();
        self.frame_buffer.extend(pixels.map(Rgb::as_u32));
        if self.display_stats {
            self.render_stats(start.elapsed());
        }
        self.should_render = false;
    }

    fn render_stats(&mut self, render_time: Duration) {
        self.render_text(
            20,
            &format!(
                "RenderTime = {} ms ({:6.3} FPS)",
                render_time.as_millis(),
                1.0 / render_time.as_secs_f64()
            ),
        );
        self.render_text(40, &format!("Scale = {:+e}", self.view.scale()));
        self.render_text(60, &format!("Offset = {:.5}", self.view.offset()));
        self.render_text(80, &format!("Seed = {:.5}", self.seed));
    }

    fn render_text(&mut self, pos_y: usize, text: &str) {
        self.font_renderer
            .draw_text(&mut self.frame_buffer, Self::TEXT_POS_X, pos_y, text);
    }
}
