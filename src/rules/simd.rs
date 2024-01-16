use nalgebra::ComplexField;

use crate::shared::Complex;
use std::{
    ops::{Add, Div, Mul, Sub},
    simd::{Mask, Simd},
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
    use itertools::Itertools;

    #[test]
    fn addition_works() {
        for (lhs, rhs) in complex_numbers_with_zero().tuple_windows() {
            check_addition(lhs, rhs);
        }
    }

    #[test]
    fn subtraction_worls() {
        for (lhs, rhs) in complex_numbers_with_zero().tuple_windows() {
            check_subtraction(lhs, rhs);
        }
    }

    #[test]
    fn multiplication_by_one_works() {
        check_multiplication(Complex::new(420.69, 13.37), Complex::new(1.0, 0.0));
    }

    #[test]
    fn multiplication_works() {
        for (lhs, rhs) in complex_numbers_with_zero().tuple_windows() {
            check_multiplication(lhs, rhs);
        }
    }

    #[test]
    fn division_by_one_works() {
        check_division(Complex::new(3.14, 2.78), Complex::new(1.0, 0.0));
    }

    #[test]
    fn division_by_nonzero_works() {
        for (lhs, rhs) in complex_numbers_without_zero().tuple_windows() {
            check_division(lhs, rhs);
        }
    }

    fn complex_numbers_without_zero() -> impl Iterator<Item = Complex> {
        (-10..10)
            .cartesian_product(-10..10)
            .map(|(x, y)| (0.33 + x as f64, 0.67 + y as f64))
            .map(|(x, y)| Complex::new(x, y))
    }

    fn complex_numbers_with_zero() -> impl Iterator<Item = Complex> {
        complex_numbers_without_zero().chain(std::iter::once(Complex::default()))
    }

    macro_rules! check_op {
        ($lhs:expr, $rhs:expr, $op:tt) => {
            let simd_lhs = SimdComplex::from_complex($lhs);
            let simd_rhs = SimdComplex::from_complex($rhs);
            assert_eq!(
                simd_lhs $op simd_rhs,
                SimdComplex::from_complex($lhs $op $rhs)
            );
        }
    }

    fn check_addition(lhs: Complex, rhs: Complex) {
        check_op!(lhs, rhs, +);
    }

    fn check_subtraction(lhs: Complex, rhs: Complex) {
        check_op!(lhs, rhs, -);
    }

    fn check_multiplication(lhs: Complex, rhs: Complex) {
        check_op!(lhs, rhs, *);
    }

    fn check_division(lhs: Complex, rhs: Complex) {
        check_op!(lhs, rhs, -);
    }
}
