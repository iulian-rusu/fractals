use std::marker::Tuple;

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

/// Trait for types which can be used by a renderer to compute RGB values.
/// The type has to be "nice" enough to allow cloning and sending to other threads, hence the `Shareable` requirement.
pub trait ColorComputer<Args: Tuple>: Fn<Args, Output = Rgb> + Shareable {}
impl<Args: Tuple, T: Fn<Args, Output = Rgb> + Shareable> ColorComputer<Args> for T {}
