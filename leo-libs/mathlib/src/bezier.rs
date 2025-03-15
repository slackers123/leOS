use corelib::types::Float;

use crate::{
    aabb::AABB,
    equations::{CubicEquation, EquationRoots},
    funcs::{approx_eq, approx_in_range_01},
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
    #[inline]
    fn pos_from_t(&self, t: Float) -> Vec2<Float> {
        // from: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_B%C3%A9zier_curves
        (1. - t) * ((1. - t) * self.p0 + t * self.p1) + t * ((1. - t) * self.p1 + t * self.p2)
    }
}

impl HorizLineIntersect for QuadraticBezier {
    fn bbox(&self) -> AABB<Float> {
        // TODO: use more optimal bounding box generation
        let mut aabb = AABB::default();
        aabb.include_vec(&self.p0);
        aabb.include_vec(&self.p1);
        aabb.include_vec(&self.p2);
        aabb
    }

    fn isect_at_y(&self, y: Float) -> Vec<Float> {
        let Self {
            mut p0,
            mut p1,
            mut p2,
        } = self.clone();

        p0.y -= y;
        p1.y -= y;
        p2.y -= y;

        let a = p0.y - 2. * p1.y + p2.y;
        let b = 2. * (p1.y - p0.y);
        let c = p0.y;

        // quadratic equation: x1,2 = (-b +/- (b.powi(2) - 4. * a * c).sqrt()) / (2. * a)
        let square_term = b.powi(2) - 4. * a * c;
        let mut res = Vec::with_capacity(2);
        if approx_eq(square_term, 0.) {
            let root = -b / (2. * a);
            if approx_in_range_01(root) {
                let x = self.pos_from_t(root).x;
                res.push(x);
                res.push(x);
            }
        } else if square_term > 0. {
            let root1 = (-b + (square_term).sqrt()) / (2. * a);
            let root2 = (-b - (square_term).sqrt()) / (2. * a);

            if approx_in_range_01(root1) {
                res.push(self.pos_from_t(root1).x);
            }
            if approx_in_range_01(root2) {
                res.push(self.pos_from_t(root2).x);
            }
        } else {
            // term in square root is negative
            // No solutions
        }

        return res;
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

impl HorizLineIntersect for CubicBezier {
    fn bbox(&self) -> AABB<Float> {
        // TODO: use more optimal bounding box generation
        let mut aabb = AABB::default();
        aabb.include_vec(&self.p0);
        aabb.include_vec(&self.p1);
        aabb.include_vec(&self.p2);
        aabb
    }

    // fn zeroed(&self, off: Vec2<Float>) -> Self {
    //     let mut res = self.clone();
    //     res.p0 += off;
    //     res.p1 += off;
    //     res.p2 += off;
    //     res.p3 += off;
    //     res
    // }

    // fn root_filter(&self, t: &Float, test_x: Float) -> bool {
    //     approx_in_range_01(t)
    // }

    fn isect_at_y(&self, y: Float) -> Vec<Float> {
        let CubicBezier {
            mut p0,
            mut p1,
            mut p2,
            mut p3,
        } = self.clone();
        p0.y -= y;
        p1.y -= y;
        p2.y -= y;
        p3.y -= y;

        let a = -p0.y + 3. * p1.y - 3. * p2.y + p3.y;
        let b = 3. * p0.y - 6. * p1.y + 3. * p2.y;
        let c = -3. * p0.y + 3. * p1.y;
        let d = p0.y;

        let roots = CubicEquation { a, b, c, d }.roots();

        return roots
            .into_iter()
            .filter(|t| approx_in_range_01(*t))
            .map(|t| self.pos_from_t(t).x)
            .collect();
    }
}
