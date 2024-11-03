use corelib::types::Float;

use crate::{
    aabb::AABB,
    equations::{CubicEquation, QuadraticEquation},
    funcs::approx_in_range_01,
    horiz_line_intersect::HorizLineIntersect,
    vectors::Vec2,
};

pub trait Bezier {
    fn pos_from_t(&self, t: Float) -> Vec2<Float>;
}

#[derive(Debug, Clone)]
pub struct QuadraticBezier {
    pub p0: Vec2<Float>,
    pub p1: Vec2<Float>,
    pub p2: Vec2<Float>,
}

impl QuadraticBezier {
    pub fn new(p0: Vec2<Float>, p1: Vec2<Float>, p2: Vec2<Float>) -> Self {
        Self { p0, p1, p2 }
    }
}

impl Bezier for QuadraticBezier {
    fn pos_from_t(&self, t: Float) -> Vec2<Float> {
        // from: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_B%C3%A9zier_curves
        (1. - t) * ((1. - t) * self.p0 + t * self.p1) + t * ((1. - t) * self.p1 + t * self.p2)
    }
}

impl HorizLineIntersect<QuadraticEquation> for QuadraticBezier {
    fn bbox(&self) -> AABB<Float> {
        // TODO: use more optimal bounding box generation
        let mut aabb = AABB::default();
        aabb.include_vec(&self.p0);
        aabb.include_vec(&self.p1);
        aabb.include_vec(&self.p2);
        aabb
    }

    fn moved_y(&self, y_off: Float) -> Self {
        let mut res = self.clone();
        res.p0.y += y_off;
        res.p1.y += y_off;
        res.p2.y += y_off;
        res
    }

    fn root_filter(&self, t: &Float, test_x: Float) -> bool {
        approx_in_range_01(t) && self.pos_from_t(*t).x < test_x
    }

    fn get_equation(&self) -> QuadraticEquation {
        let a = self.p0.y - 2. * self.p1.y + self.p2.y;
        let b = 2. * (self.p1.y - self.p0.y);
        let c = self.p0.y;

        QuadraticEquation { a, b, c }
    }
}

#[derive(Debug, Clone)]
pub struct CubicBezier {
    pub p0: Vec2<Float>,
    pub p1: Vec2<Float>,
    pub p2: Vec2<Float>,
    pub p3: Vec2<Float>,
}

impl CubicBezier {
    pub fn new(p0: Vec2<Float>, p1: Vec2<Float>, p2: Vec2<Float>, p3: Vec2<Float>) -> Self {
        Self { p0, p1, p2, p3 }
    }
}

impl Bezier for CubicBezier {
    fn pos_from_t(&self, t: Float) -> Vec2<Float> {
        // from: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Cubic_B%C3%A9zier_curves
        (1. - t).powi(3) * self.p0
            + 3. * (1. - t).powi(2) * t * self.p1
            + 3. * (1. - t) * t.powi(2) * self.p2
            + t.powi(3) * self.p3
    }
}

impl HorizLineIntersect<CubicEquation> for CubicBezier {
    fn bbox(&self) -> AABB<Float> {
        // TODO: use more optimal bounding box generation
        let mut aabb = AABB::default();
        aabb.include_vec(&self.p0);
        aabb.include_vec(&self.p1);
        aabb.include_vec(&self.p2);
        aabb
    }

    fn moved_y(&self, y_off: Float) -> Self {
        let mut res = self.clone();
        res.p0.y += y_off;
        res.p1.y += y_off;
        res.p2.y += y_off;
        res.p3.y += y_off;
        res
    }

    fn root_filter(&self, t: &Float, test_x: Float) -> bool {
        approx_in_range_01(t) && self.pos_from_t(*t).x < test_x
    }

    fn get_equation(&self) -> CubicEquation {
        let CubicBezier { p0, p1, p2, p3 } = self;
        let a = -p0.y + 3. * p1.y - 3. * p2.y + p3.y;
        let b = 3. * p0.y - 6. * p1.y + 3. * p2.y;
        let c = -3. * p0.y + 3. * p1.y;
        let d = p0.y;

        CubicEquation { a, b, c, d }
    }
}
