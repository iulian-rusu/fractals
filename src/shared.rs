use crate::color::Rgb;

pub type Complex = nalgebra::Complex<f64>;

pub mod directon {
    use crate::shared::Complex;

    pub const UP: Complex = Complex::new(0.0, 1.0);
    pub const DOWN: Complex = Complex::new(0.0, -1.0);
    pub const LEFT: Complex = Complex::new(1.0, 0.0);
    pub const RIGHT: Complex = Complex::new(-1.0, 0.0);
}

pub trait Shareable: Send + Clone + 'static {}
impl<T: Send + Clone + 'static> Shareable for T {}

/// Computes the RGB color asigned to a given complex number
pub trait ColorComputer: Fn(Complex) -> Rgb + Shareable {}
impl<T: Fn(Complex) -> Rgb + Shareable> ColorComputer for T {}

/// Computes the number of iterations starting from a pair (seed, z) of complex numbers
pub trait IterationComputer: Fn(Complex, Complex) -> u8 + Shareable {}
impl<T: Fn(Complex, Complex) -> u8 + Shareable> IterationComputer for T {}