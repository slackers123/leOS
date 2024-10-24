use corelib::types::Float;

use crate::consts::EPSILON;

/// get the square root of a
pub fn sqrt(a: Float) -> Float {
    a.sqrt()
}

/// real-cuberoots-only
pub fn real_cuberoot(a: Float) -> Float {
    if a < 0. {
        -(-a).cbrt()
    } else {
        a.cbrt()
    }
}

/// is a approximately equal to b.
///
/// if a is in range of an epsilon
pub fn approx_eq(a: Float, b: Float) -> bool {
    let diff = a - b;
    diff < EPSILON && diff > -EPSILON
}

/// is x approximately in range a..b
pub fn approx_in_range(x: &Float, a: Float, b: Float) -> bool {
    let base = x - a;
    let diff = b - a;
    base > -EPSILON && base < diff + EPSILON
}

/// is x approximately in range 0..1
pub fn approx_in_range_01(x: &Float) -> bool {
    approx_in_range(x, 0., 1.)
}
