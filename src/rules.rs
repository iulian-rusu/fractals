use crate::simd::{Array, SimdComplex, SimdCounter, SimdDouble, SIMD_LEN};
use std::simd::cmp::SimdPartialOrd;

pub const MAX_ITERS: u8 = u8::MAX;

const EPSILON: SimdDouble = SimdDouble::from_array([1e-10; SIMD_LEN]);
const ESCAPE_RADIUS_SQUARED: SimdDouble = SimdDouble::from_array([4.0; SIMD_LEN]);

#[allow(dead_code)]
pub fn julia(mut z: SimdComplex, c: SimdComplex) -> Array<u8> {
    let mut cnt = SimdCounter::new();
    for _ in 0..MAX_ITERS {
        cnt.increment_where(z.norm_squared().simd_lt(ESCAPE_RADIUS_SQUARED));
        if !cnt.modified() {
            break;
        }
        z = z * z + c;
    }
    cnt.counts()
}

#[allow(dead_code)]
pub fn mandelbrot(c: SimdComplex) -> Array<u8> {
    julia(SimdComplex::default(), c)
}

#[allow(dead_code)]
pub fn nova(
    mut z: SimdComplex,
    c: SimdComplex,
    f: impl Fn(SimdComplex) -> SimdComplex,
    df: impl Fn(SimdComplex) -> SimdComplex,
) -> Array<u8> {
    let mut cnt = SimdCounter::new();
    for _ in 0..MAX_ITERS {
        let z_next = z - f(z) / df(z) + c;
        cnt.increment_where((z_next - z).norm_squared().simd_ge(EPSILON));
        if !cnt.modified() {
            break;
        }
        z = z_next;
    }
    cnt.counts()
}

#[allow(dead_code)]
pub fn newton(
    z: SimdComplex,
    f: impl Fn(SimdComplex) -> SimdComplex,
    df: impl Fn(SimdComplex) -> SimdComplex,
) -> Array<u8> {
    nova(z, SimdComplex::default(), f, df)
}
