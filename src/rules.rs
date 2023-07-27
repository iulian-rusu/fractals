use nalgebra::Normed;

use crate::shared::Complex;

#[allow(dead_code)]
pub fn julia(c: Complex, mut z: Complex) -> u8 {
    for i in 0..u8::MAX {
        if z.norm_squared() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    u8::MAX
}

#[allow(dead_code)]
pub fn mandelbrot(c: Complex) -> u8 {
    let mut z = Complex::default();
    for i in 0..u8::MAX {
        if z.norm_squared() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    u8::MAX
}
