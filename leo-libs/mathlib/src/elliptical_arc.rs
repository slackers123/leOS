use corelib::types::{Float, Uint};

use crate::vectors::Vec2;

pub struct EllipticalArc {
    pub r: Vec2<Float>,
    pub rot: Float,
    pub c: Vec2<Float>,
}

impl EllipticalArc {
    // pub fn isect(&self, test_point: Vec2<Float>) -> Uint {}
}
