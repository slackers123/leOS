use corelib::types::Float;

use crate::{
    aabb::AABB, funcs::approx_eq, horiz_line_intersect::HorizLineIntersect, vectors::Vec2,
};

#[derive(Debug, Clone)]
pub struct LineSegment {
    pub start: Vec2<Float>,
    pub end: Vec2<Float>,
}

impl LineSegment {
    pub fn new(start: Vec2<Float>, end: Vec2<Float>) -> Self {
        Self { start, end }
    }
}

impl HorizLineIntersect for LineSegment {
    fn bbox(&self) -> crate::aabb::AABB<Float> {
        let mut bbox = AABB::default();
        bbox.include_vec(&self.start);
        bbox.include_vec(&self.end);
        bbox
    }

    fn isect_at_y(&self, y: Float) -> Vec<Float> {
        // This is not needed if the bbox check was performed
        // if y < Float::min(self.end.y, self.start.y) || y > Float::max(self.end.y, self.start.y) {
        //     println!("{y}");
        //     return vec![];
        // }
        let d = self.end - self.start;
        let k = d.x / d.y;
        // y = kx + d
        if approx_eq(k, 0.) {
            if approx_eq(self.start.y, y) {
                return vec![self.start.x, self.end.x];
            } else {
                return vec![];
            }
        } else {
            let d = (self.start.y - y) - k * self.start.x;
            return vec![-(d / k)];
        }
    }
}
