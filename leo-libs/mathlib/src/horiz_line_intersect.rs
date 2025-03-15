use corelib::types::Float;

use crate::aabb::AABB;

pub trait HorizLineIntersect {
    fn bbox(&self) -> AABB<Float>;
    fn isect_at_y(&self, y: Float) -> Vec<Float>;
}
