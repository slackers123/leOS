use corelib::types::Float;

use crate::{
    aabb::AABB, equations::LinearEquation, horiz_line_intersect::HorizLineIntersect, vectors::Vec2,
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

impl HorizLineIntersect<LinearEquation> for LineSegment {
    fn bbox(&self) -> crate::aabb::AABB<Float> {
        let mut bbox = AABB::default();
        bbox.include_vec(&self.start);
        bbox.include_vec(&self.end);
        bbox
    }

    fn moved_y(&self, y_off: Float) -> Self {
        let mut res = self.clone();
        res.start.y += y_off;
        res.end.y += y_off;
        res
    }

    fn get_equation(&self) -> LinearEquation {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let k = dx / dy;
        let d = self.start.y - k * self.start.x;
        LinearEquation { a: k, b: d }
    }

    fn root_filter(&self, t: &Float, test_x: Float) -> bool {
        -t < test_x
    }
}
