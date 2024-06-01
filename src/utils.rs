use std::marker::Tuple;

pub type Complex = nalgebra::Complex<f64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

/// Trait for functions that can be shared and invoked by multiple threads.
pub trait FnSync<Args: Tuple>: Fn<Args> + Sync + Send {}
impl<Args: Tuple, F> FnSync<Args> for F
where
    F: Fn<Args> + Sync + Send,
    F::Output: Send,
{
}
