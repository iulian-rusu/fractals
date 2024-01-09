use nalgebra::{ComplexField, Normed};

use crate::shared::Complex;

const MAX_ITERS: u8 = u8::MAX;
const EPSILON: f64 = 1e-5;

#[allow(dead_code)]
pub fn julia(mut z: Complex, c: Complex) -> u8 {
    for i in 0..MAX_ITERS {
        if z.norm_squared() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    u8::MAX
}

#[allow(dead_code)]
pub fn mandelbrot(c: Complex) -> u8 {
    julia(Complex::default(), c)
}

#[allow(dead_code)]
pub fn nova(
    mut z: Complex,
    c: Complex,
    f: impl Fn(Complex) -> Complex,
    df: impl Fn(Complex) -> Complex,
) -> u8 {
    for i in 0..MAX_ITERS {
        let z_next = z - f(z) / df(z) + c;
        if (z_next - z).abs() < EPSILON {
            return i;
        }
        z = z_next;
    }
    u8::MAX
}

#[allow(dead_code)]
pub fn newton(
    z: Complex,
    f: impl Fn(Complex) -> Complex,
    df: impl Fn(Complex) -> Complex,
) -> u8 {
    nova(z, Complex::default(), f, df)
}
