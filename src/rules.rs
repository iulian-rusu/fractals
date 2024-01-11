#![allow(dead_code)]
use crate::shared::Complex;
use nalgebra::{ComplexField, Normed};

const MAX_ITERS: u8 = u8::MAX;
const EPSILON: f64 = 1e-5;

pub fn julia(mut z: Complex, c: Complex) -> u8 {
    for i in 0..MAX_ITERS {
        if z.norm_squared() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    u8::MAX
}

pub fn mandelbrot(c: Complex) -> u8 {
    julia(Complex::default(), c)
}

pub fn nova<F1, F2>(mut z: Complex, c: Complex, f: F1, df: F2) -> u8
where
    F1: Fn(Complex) -> Complex,
    F2: Fn(Complex) -> Complex,
{
    for i in 0..MAX_ITERS {
        let z_next = z - f(z) / df(z) + c;
        if (z_next - z).abs() < EPSILON {
            return i;
        }
        z = z_next;
    }
    u8::MAX
}

pub fn newton<F1, F2>(z: Complex, f: F1, df: F2) -> u8
where
    F1: Fn(Complex) -> Complex,
    F2: Fn(Complex) -> Complex,
{
    nova(z, Complex::default(), f, df)
}
