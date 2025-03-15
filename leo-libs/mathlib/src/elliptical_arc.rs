use std::f32::consts::PI;

use corelib::types::Float;

use crate::{
    aabb::AABB,
    angles::Rad,
    funcs::approx_in_range_no_order,
    horiz_line_intersect::HorizLineIntersect,
    matrix::{Fmat2, Mat2},
    vectors::Vec2,
};

#[derive(Debug, Clone)]
pub struct EllipticalArc {
    pub start: Vec2<Float>,
    pub end: Vec2<Float>,
    pub r: Vec2<Float>,
    pub rot: Float,
    pub large_arc_flag: bool,
    pub sweep_flag: bool,
}

impl EllipticalArc {
    pub fn to_equation(self) -> EllipticalArcEquation {
        let mut r = self.r;
        let start_prime = (Mat2::new([
            [self.rot.cos(), self.rot.sin()],
            [-self.rot.sin(), self.rot.cos()],
        ]) * ((self.start - self.end) / 2.).to_mat())
        .as_vec();

        if r.x == 0. || r.y == 0. {
            panic!("should be treated as line");
        }

        r = Vec2::new(r.x.abs(), r.y.abs());

        let big_a = start_prime.x.powi(2) / r.x.powi(2) + start_prime.y.powi(2) / r.y.powi(2);
        let mut radicand = None;
        if big_a > 1. {
            r = Vec2::new(r.x * big_a.sqrt(), r.y * big_a.sqrt());
            radicand = Some(0.);
        }

        let radicand = radicand.unwrap_or(
            (r.x.powi(2) * r.y.powi(2)
                - r.x.powi(2) * start_prime.y.powi(2)
                - r.y.powi(2) * start_prime.x.powi(2))
                / (r.x.powi(2) * start_prime.y.powi(2) + r.y.powi(2) * start_prime.x.powi(2)),
        );

        if radicand < 0. {
            panic!(
                "something went seriously wrong.  \
                (see ending of this section of the spec: \
                https://www.w3.org/TR/SVG2/implnote.html#ArcCorrectionOutOfRangeRadii)"
            )
        }

        let c_prime = if self.large_arc_flag == self.sweep_flag {
            -1.
        } else {
            1.
        } * radicand.sqrt()
            * Vec2::new((r.x * start_prime.y) / r.y, -(r.y * start_prime.x) / r.x);

        let center = (Mat2::new([
            [self.rot.cos(), -self.rot.sin()],
            [self.rot.sin(), self.rot.cos()],
        ]) * c_prime.to_mat())
        .as_vec()
            + ((self.start + self.end) / 2.);

        let end_point = Vec2::new(
            (start_prime.x - c_prime.x) / r.x,
            (start_prime.y - c_prime.y) / r.y,
        );

        let start_angle = Vec2::new(1.0, 0.0).angle_to(&end_point);

        let angle_delta = end_point.angle_to(&Vec2::new(
            (-start_prime.x - c_prime.x) / r.x,
            (-start_prime.y - c_prime.y) / r.y,
        ));

        let mut angle_delta = angle_delta % (PI * 2.);

        if !self.sweep_flag && angle_delta > 0. {
            angle_delta -= PI * 2.;
        } else if self.sweep_flag && angle_delta < 0. {
            angle_delta += PI * 2.;
        }

        return EllipticalArcEquation {
            r,
            rot: self.rot,
            c: center,
            start_angle: Rad::new(start_angle),
            angle_delta,
        };
    }
}

#[derive(Debug, Clone)]
pub struct EllipticalArcEquation {
    pub r: Vec2<Float>,
    pub rot: Float,
    pub c: Vec2<Float>,
    pub start_angle: Rad,
    pub angle_delta: Float,
}

impl EllipticalArcEquation {
    pub fn get_pos_from_angle(&self, angle: Float) -> Vec2<Float> {
        Vec2::from_mat(
            Fmat2::new([
                [self.rot.cos(), -self.rot.sin()],
                [self.rot.sin(), self.rot.cos()],
            ]) * Vec2::new(self.r.x * angle.cos(), self.r.y * angle.sin()).to_mat()
                + self.c.to_mat(),
        )
    }

    pub fn get_angle_from_pos(&self, pos: Vec2<Float>) -> Rad {
        let pos = pos - self.c;
        let new_pos = Vec2::from_mat(
            Fmat2::new([
                [self.rot.cos(), -self.rot.sin()],
                [self.rot.sin(), self.rot.cos()],
            ])
            .inverse()
                * pos.to_mat(),
        );
        let angle = new_pos.y.atan2(new_pos.x);
        Rad::new(angle)
    }
}

impl HorizLineIntersect for EllipticalArcEquation {
    fn bbox(&self) -> crate::aabb::AABB<Float> {
        // FIXME: this is not correct because it doesnt account for the rotation
        let off_x = self.r.x / 2.;
        let off_y = self.r.y / 2.;
        let mut bbox = AABB::default();
        bbox.include_vec(&Vec2::new(self.c.x + off_x, self.c.y + off_y));
        bbox.include_vec(&Vec2::new(self.c.x - off_x, self.c.y + off_y));
        bbox.include_vec(&Vec2::new(self.c.x + off_x, self.c.y - off_y));
        bbox.include_vec(&Vec2::new(self.c.x - off_x, self.c.y - off_y));
        bbox
    }

    /// adapted from: http://quickcalcbasic.com/ellipse%20line%20intersection.pdf
    /// equations: 9a, 9b, 9c
    fn isect_at_y(&self, y_orig: Float) -> Vec<Float> {
        let EllipticalArcEquation {
            r: Vec2 { x: h, y: v },
            rot: alp,
            c: Vec2 { x, y: c_y },
            start_angle,
            angle_delta,
        } = self;

        let y = y_orig - c_y;

        let a = v.powi(2) * alp.cos().powi(2) + h.powi(2) * alp.sin().powi(2);
        let b = 2. * y * alp.cos() * alp.sin() * (v.powi(2) - h.powi(2));
        let c = y.powi(2) * (v.powi(2) * alp.sin().powi(2) + h.powi(2) * alp.cos().powi(2))
            - h.powi(2) * v.powi(2);

        let d = b.powi(2) - 4. * a * c;
        let denom = 2. * a;

        if d < 0. || denom == 0. {
            return vec![];
        }

        let x1 = (-b + d.sqrt()) / denom;
        let x2 = (-b - d.sqrt()) / denom;

        // return vec![x + x1, x + x2];

        let a1 = self.get_angle_from_pos(Vec2::new(x + x1, y_orig));
        let a2 = self.get_angle_from_pos(Vec2::new(x + x2, y_orig));

        let mut res = Vec::with_capacity(2);
        if approx_in_range_no_order(
            a1.as_float(),
            start_angle.as_float(),
            start_angle.as_float() + *angle_delta,
        ) {
            res.push(x + x1);
        }
        if approx_in_range_no_order(
            a2.as_float(),
            start_angle.as_float(),
            start_angle.as_float() + *angle_delta,
        ) {
            res.push(x + x2);
        }
        res
    }
}
