use corelib::types::{Float, Uint};

use crate::{equations::EquationRoots, horiz_line_intersect::HorizLineIntersect, vectors::Vec2};

pub struct EllipticalArc {
    pub r: Vec2<Float>,
    pub rot: Float,
    pub c: Vec2<Float>,
    pub start_angle: Float,
    pub angle_delta: Float,
}

impl EllipticalArc {
    /// adapted from: http://quickcalcbasic.com/ellipse%20line%20intersection.pdf
    /// equations: 9a, 9b, 9c
    pub fn isect(&self, test_point: Vec2<Float>) -> Uint {
        // TODO: OPTIMIZATION - (maybe even outside of this function) calculate start and end point
        //       of the arc and use that for bounding box check
        let new_tp = test_point - self.c;
        let EllipticalArc {
            r: Vec2 { x: h, y: v },
            rot,
            c: _,
            start_angle,
            angle_delta,
        } = self;
        let y = new_tp.y;

        let a = v.powi(2) * rot.cos().powi(2) + h.powi(2) * rot.sin().powi(2);
        let b = 2. * y * rot.cos() * rot.sin() * (v.powi(2) - h.powi(2));
        let c = y.powi(2) * (v.powi(2) * rot.sin().powi(2) + h.powi(2) * rot.cos().powi(2))
            - h.powi(2) * v.powi(2);

        let d = b.powi(2) - 4. * a * c;
        let denom = 2. * a;

        if d < 0. || denom == 0. {
            return 0;
        }

        let x1 = (-b + d.sqrt()) / denom;
        let x2 = (-b - d.sqrt()) / denom;

        if x1 <= new_tp.x && x2 <= new_tp.x {
            return 2;
        } else if x1 <= new_tp.x || x2 <= new_tp.x {
            return 1;
        }
        return 0;
    }
}

impl EquationRoots for EllipticalArc {
    /// adapted from: http://quickcalcbasic.com/ellipse%20line%20intersection.pdf
    /// equations: 9a, 9b, 9c
    fn roots(&self) -> Vec<Float> {
        // TODO: OPTIMIZATION - (maybe even outside of this function) calculate start and end point
        //       of the arc and use that for bounding box check
        let EllipticalArc {
            r: Vec2 { x: h, y: v },
            rot,
            c: _,
            start_angle,
            angle_delta,
        } = self;

        let a = v.powi(2) * rot.cos().powi(2) + h.powi(2) * rot.sin().powi(2);
        let c = -h.powi(2) * v.powi(2);

        let d = -4. * a * c;
        let denom = 2. * a;

        if d < 0. || denom == 0. {
            return vec![];
        }

        let x1 = (d.sqrt()) / denom;
        let x2 = (-d.sqrt()) / denom;

        if x1 == x2 {
            return vec![x1];
        }

        return vec![x1, x2];
    }
}

impl HorizLineIntersect<EllipticalArc> for EllipticalArc {}
