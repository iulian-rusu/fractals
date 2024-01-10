use crate::color::Rgb;
use std::marker::Tuple;

pub type Complex = nalgebra::Complex<f64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction(Complex);

impl Direction {
    pub const UP: Direction = Direction(Complex::new(0.0, 1.0));
    pub const DOWN: Direction = Direction(Complex::new(0.0, -1.0));
    pub const RIGHT: Direction = Direction(Complex::new(1.0, 0.0));
    pub const LEFT: Direction = Direction(Complex::new(-1.0, 0.0));

    pub fn as_complex(&self) -> Complex {
        self.0
    }
}

pub trait Shareable: Send + Clone + 'static {}
impl<T: Send + Clone + 'static> Shareable for T {}

/// Trait for types which can be used by a renderer to compute RGB values.
/// The type has to be "nice" enough to allow cloning and sending to other threads, hence the `Shareable` requirement.
pub trait ColorComputer<Args: Tuple>: Fn<Args, Output = Rgb> + Shareable {}
impl<Args: Tuple, T: Fn<Args, Output = Rgb> + Shareable> ColorComputer<Args> for T {}
