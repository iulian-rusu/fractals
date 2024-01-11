use crate::color::Rgb;
use std::marker::Tuple;

pub type Complex = nalgebra::Complex<f64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn as_complex(self) -> Complex {
        match self {
            Direction::Up => Complex::new(0.0, 1.0),
            Direction::Down => Complex::new(0.0, -1.0),
            Direction::Left => Complex::new(-1.0, 0.0),
            Direction::Right => Complex::new(1.0, 0.0),
        }
    }
}

pub trait Shareable: Send + Clone + 'static {}
impl<T: Send + Clone + 'static> Shareable for T {}

/// Trait for types which can be used by a renderer to compute RGB values.
/// The type has to be "nice" enough to allow cloning and sending to other threads, hence the `Shareable` requirement.
pub trait ColorComputer<Args: Tuple>: Fn<Args, Output = Rgb> + Shareable {}
impl<Args: Tuple, T: Fn<Args, Output = Rgb> + Shareable> ColorComputer<Args> for T {}
