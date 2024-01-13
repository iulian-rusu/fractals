use nalgebra::ComplexField;

use crate::shared::Complex;
use std::{
    ops::{Add, Div, Mul, Sub},
    simd::{Simd, Mask},
};

/// SIMD parallelization factor, chosen empirically.
pub const SIMD_LANES: usize = 8;

/// Array with size compatible with SIMD lane count
pub type Array<T> = [T; SIMD_LANES];

pub type SimdDouble = Simd<f64, SIMD_LANES>;

#[derive(Debug, Clone)]
pub struct SimdCounter {
    counts: Simd<i64, SIMD_LANES>,
    modified: bool,
}

impl SimdCounter {
    pub fn new() -> Self {
        Self {
            counts: Simd::<i64, SIMD_LANES>::from_array([0; SIMD_LANES]),
            modified: false,
        }
    }

    pub fn increment_where(&mut self, mask: Mask<i64, SIMD_LANES>) {
        // Subtract because true is converted to -1
        self.counts = self.counts - mask.to_int();
        self.modified = mask.any();
    }

    pub fn counts(&self) -> Array<u8> {
        self.counts.as_array().map(|x| x as u8)
    }

    pub fn modified(&self) -> bool {
        self.modified
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimdComplex {
    pub re: SimdDouble,
    pub im: SimdDouble,
}

impl SimdComplex {
    pub const LANES: usize = SimdDouble::LANES;

    pub fn new(re: f64, im: f64) -> Self {
        Self {
            re: SimdDouble::from_array([re; Self::LANES]),
            im: SimdDouble::from_array([im; Self::LANES]),
        }
    }

    pub fn from_complex(z: Complex) -> Self {
        Self::new(z.real(), z.imaginary())
    }

    pub fn norm_squared(&self) -> SimdDouble {
        self.re * self.re + self.im * self.im
    }
}

impl Default for SimdComplex {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Add for SimdComplex {
    type Output = SimdComplex;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add<f64> for SimdComplex {
    type Output = SimdComplex;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re + SimdDouble::from_array([rhs; Self::LANES]),
            im: self.im,
        }
    }
}

impl Sub for SimdComplex {
    type Output = SimdComplex;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<f64> for SimdComplex {
    type Output = SimdComplex;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re - SimdDouble::from_array([rhs; Self::LANES]),
            im: self.im,
        }
    }
}

impl Mul for SimdComplex {
    type Output = SimdComplex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl Mul<f64> for SimdComplex {
    type Output = SimdComplex;

    fn mul(self, rhs: f64) -> Self::Output {
        let factor = SimdDouble::from_array([rhs; Self::LANES]);
        Self {
            re: self.re * factor,
            im: self.im * factor,
        }
    }
}

impl Div for SimdComplex {
    type Output = SimdComplex;

    fn div(self, rhs: Self) -> Self::Output {
        let norm_sqr = rhs.norm_squared();
        let re = self.re * rhs.re + self.im * rhs.im;
        let im = self.im * rhs.re - self.re * rhs.im;
        Self {
            re: re / norm_sqr,
            im: im / norm_sqr,
        }
    }
}

impl Div<f64> for SimdComplex {
    type Output = SimdComplex;

    fn div(self, rhs: f64) -> Self::Output {
        let factor = SimdDouble::from_array([rhs; Self::LANES]);
        Self {
            re: self.re / factor,
            im: self.im / factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{rules::simd::SimdComplex, shared::Complex};

    #[test]
    fn division() {
        check_division(Complex::new(0.0, 0.0), Complex::new(4.0, 7.0));
        check_division(Complex::new(1.0, 0.0), Complex::new(1.0, 0.0));
        check_division(Complex::new(0.0, 1.0), Complex::new(1.0, 0.0));
        check_division(Complex::new(6.0, 5.0), Complex::new(0.0, 1.0));
        check_division(Complex::new(2.0, 3.0), Complex::new(4.0, 6.0));
        check_division(Complex::new(-3.0, 5.0), Complex::new(3.0, -5.0));
        check_division(Complex::new(2.0, 3.0), Complex::new(-5.0, 4.0));
    }

    fn check_division(lhs: Complex, rhs: Complex) {
        let simd_lhs = SimdComplex::from_complex(lhs);
        let simd_rhs = SimdComplex::from_complex(rhs);
        assert_eq!(simd_lhs / simd_rhs, SimdComplex::from_complex(lhs / rhs));
    }
}
