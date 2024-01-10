use crate::shared::{Complex, Direction};

#[derive(Debug, Clone)]
pub struct Viewport {
    width: usize,
    height: usize,
    offset: Complex,
    scale: f64,
}

impl Viewport {
    const INITIAL_OFFSET: Complex = Complex::new(0.0, 0.0);
    const BASE_OFFSET_DELTA: f64 = 0.025;
    const INITIAL_SCALE: f64 = 1.0;
    const ZOOM_FACTOR: f64 = 0.85;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            offset: Self::INITIAL_OFFSET,
            scale: Self::INITIAL_SCALE,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> Complex {
        self.offset
    }

    /// Creates a function that maps viewport coordinates to Complex plane coordinates
    pub fn mapper(&self) -> impl Fn(usize, usize) -> Complex {
        let smallest_dimension = std::cmp::min(self.width, self.height) as f64;
        let pixel_scale = self.scale / smallest_dimension;
        let half_width = self.width as f64 * 0.5;
        let half_height = self.height as f64 * 0.5;
        let offset = self.offset;

        move |x, y| {
            let re = pixel_scale * (x as f64 - half_width);
            let im = pixel_scale * (half_height - y as f64);
            Complex::new(re, im) + offset
        }
    }

    pub fn move_towards(&mut self, direction: Direction) {
        self.offset += direction.as_complex() * Self::BASE_OFFSET_DELTA * self.scale;
    }

    pub fn zoom_out(&mut self) {
        self.scale *= Self::ZOOM_FACTOR;
    }

    pub fn zoom_in(&mut self) {
        self.scale /= Self::ZOOM_FACTOR;
    }

    pub fn reset(&mut self) {
        self.scale = Self::INITIAL_SCALE;
        self.offset = Self::INITIAL_OFFSET;
    }
}
