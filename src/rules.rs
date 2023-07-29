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
pub fn nova<const N: u32>(mut z: Complex, c: Complex) -> u8 {
    let coef = (N - 1) as f64;
    for i in 0..MAX_ITERS {
        let z_next = z - (z.powu(N) - 1.0) / (coef * z.powu(N - 1)) + c;
        if (z_next - z).abs() < EPSILON {
            return i;
        }
        z = z_next;
    }
    u8::MAX
}

#[allow(dead_code)]
pub fn newton<const N: u32>(z: Complex) -> u8 {
    nova::<N>(z, Complex::default())
}
