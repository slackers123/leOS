use corelib::types::Float;

use crate::{number::Number, vectors::Vec2};

#[derive(Debug, Clone, Copy)]
pub struct AABB<T: Number> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl AABB<Float> {
    pub fn include_vec(&mut self, point: &Vec2<Float>) {
        if point.x < self.min.x {
            self.min.x = point.x
        };
        if point.y < self.min.y {
            self.min.y = point.y
        };
        if point.x > self.max.x {
            self.max.x = point.x
        };
        if point.y > self.max.y {
            self.max.y = point.y
        };
    }
}

impl Default for AABB<Float> {
    fn default() -> Self {
        AABB {
            min: Vec2::INFINITY,
            max: Vec2::NEG_INFINITY,
        }
    }
}
