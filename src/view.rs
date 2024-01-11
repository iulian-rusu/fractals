use crate::shared::{Complex, Direction};

/// Defines a rectangular view of the Complex plane.
/// A zero offset means the view is centered on (0, 0).
#[derive(Debug, Clone)]
pub struct ComplexPlaneView {
    width: usize,
    height: usize,
    offset: Complex,
    scale: f64,
}

impl ComplexPlaneView {
    const INITIAL_OFFSET: Complex = Complex::new(0.0, 0.0);
    const BASE_OFFSET_STEP: f64 = 0.025;
    const INITIAL_SCALE: f64 = 1.0;
    const SCALE_FACTOR: f64 = 0.85;

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

    /// Creates a function that maps pixel coordinates to Complex plane coordinates
    pub fn pixel_mapper(&self) -> impl Fn(usize, usize) -> Complex {
        let smallest_dimension = std::cmp::min(self.width, self.height) as f64;
        let pixel_scale = self.scale / smallest_dimension;
        let half_width = self.width as f64 * 0.5;
        let half_height = self.height as f64 * 0.5;
        let offset = self.offset;

        move |x, y| {
            let x_centered = x as f64 - half_width;
            let y_centered = half_height - y as f64;
            let re = pixel_scale * x_centered;
            let im = pixel_scale * y_centered;
            Complex::new(re, im) + offset
        }
    }

    pub fn translate(&mut self, direction: Direction) {
        self.offset += direction.as_complex() * Self::BASE_OFFSET_STEP * self.scale;
    }

    pub fn zoom_out(&mut self) {
        self.scale *= Self::SCALE_FACTOR;
    }

    pub fn zoom_in(&mut self) {
        self.scale /= Self::SCALE_FACTOR;
    }

    pub fn reset(&mut self) {
        self.scale = Self::INITIAL_SCALE;
        self.offset = Self::INITIAL_OFFSET;
    }
}
