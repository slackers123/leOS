use corelib::types::Float;

use crate::{aabb::AABB, vectors::Vec2};

pub trait Bezier {
    fn pos_from_t(&self, t: Float) -> Vec2<Float>;
    fn bbox(&self) -> AABB<Float>;
    fn move_y(&mut self, y_off: Float);
}

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
    fn bbox(&self) -> AABB<Float> {
        // TODO: use more optimal bounding box generation
        let mut aabb = AABB::default();
        aabb.include_vec(&self.p0);
        aabb.include_vec(&self.p1);
        aabb.include_vec(&self.p2);
        aabb
    }
    fn move_y(&mut self, y_off: Float) {
        self.p0.y += y_off;
        self.p1.y += y_off;
        self.p2.y += y_off;
    }
}

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
    fn bbox(&self) -> AABB<Float> {
        // TODO: use more optimal bounding box generation
        let mut aabb = AABB::default();
        aabb.include_vec(&self.p0);
        aabb.include_vec(&self.p1);
        aabb.include_vec(&self.p2);
        aabb.include_vec(&self.p3);
        aabb
    }
    fn move_y(&mut self, y_off: Float) {
        self.p0.y += y_off;
        self.p1.y += y_off;
        self.p2.y += y_off;
        self.p3.y += y_off;
    }
}
