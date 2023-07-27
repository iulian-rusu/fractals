pub type Complex = nalgebra::Complex<f64>;

pub mod directon {
    use crate::shared::Complex;

    pub const UP: Complex = Complex::new(0.0, 1.0);
    pub const DOWN: Complex = Complex::new(0.0, -1.0);
    pub const LEFT: Complex = Complex::new(1.0, 0.0);
    pub const RIGHT: Complex = Complex::new(-1.0, 0.0);
}