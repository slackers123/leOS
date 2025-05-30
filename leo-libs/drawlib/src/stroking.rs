#![allow(non_snake_case)]
//! Based on Polar Stroking: https://dl.acm.org/doi/abs/10.1145/3386569.3392458

use std::{f32::consts::PI, usize};

use corelib::types::Float;
use mathlib::{
    angles::{Rad, PI2},
    color::ColA,
    elliptical_arc::EllipticalArcEquation,
    equations::{EquationRoots, QuadraticEquation},
    funcs::approx_in_range_01,
    intersect::intersect_two_lines,
    matrix::Mat,
    vectors::Vec2,
};
use renderlib::{
    material::Material,
    primitive::{Mesh, MeshType, Primitive},
};

use crate::drawable::Drawable;

pub const QUALITY_DEG: Float = 10.0;
const QUALITY: Float = QUALITY_DEG / 180.0 * PI;

#[derive(Debug)]
pub enum JoinType {
    None,
    Miter,
    Bevel,
    Round,
    MiterTruncate(Float),
    MiterRevert(Float),
}

pub struct Path {
    pub segs: Vec<PathSeg>,
    pub join_type: JoinType,
    pub width: Float,
}

impl Drawable for Path {
    fn to_primitives(self) -> Vec<Primitive> {
        let mut res_vertices = vec![];
        for (i, seg) in self.segs.iter().enumerate() {
            let res = stroke(seg, self.width)
                .into_iter()
                .map(|v| (v.0, v.1))
                .collect::<Vec<_>>();

            for i in 0..res.len() - 1 {
                res_vertices.push(res[i].0);
                res_vertices.push(res[i].1);
                res_vertices.push(res[i + 1].0);

                res_vertices.push(res[i].1);
                res_vertices.push(res[i + 1].0);
                res_vertices.push(res[i + 1].1);
            }

            if self.segs.len() > 1 && i > 0 {
                let dir2 = seg.init_norm_grad();
                let dir1 = self.segs[i - 1].term_norm_grad();
                if dir1 == dir2 {
                    continue;
                }
                let angle = dir2.angle_to(&dir1);

                let (p1, p2) = if angle > 0.0 {
                    (seg.init_p(self.width), self.segs[i - 1].term_p(self.width))
                } else {
                    (seg.init_n(self.width), self.segs[i - 1].term_n(self.width))
                };

                match self.join_type {
                    JoinType::None => {}
                    JoinType::Miter => {
                        let p3 = intersect_two_lines(p1, dir1, p2, dir2).unwrap();

                        // FIXME: this is slightly off (rounding)

                        res_vertices.push(p1);
                        res_vertices.push(p2);
                        res_vertices.push(p3);

                        let p3 = seg.generator(0.0);

                        res_vertices.push(p1);
                        res_vertices.push(p2);
                        res_vertices.push(p3);
                    }
                    JoinType::Bevel => {
                        let p3 = seg.generator(0.0);

                        res_vertices.push(p1);
                        res_vertices.push(p2);
                        res_vertices.push(p3);
                    }

                    JoinType::Round => {
                        let J = (angle / QUALITY).ceil() as usize;

                        let center = seg.generator(0.0);

                        let mut last = p1;
                        for i in 1..=J {
                            let t = i as Float / J as Float;
                            let n_angle = dir1.x_angle() + (angle * t);
                            let new = Vec2::dir(n_angle) * (self.width / 2.0);
                            res_vertices.push(last);
                            res_vertices.push(center);
                            res_vertices.push(center + new);
                            last = new;
                        }
                        println!("{:?}", p1);
                    }
                    _ => todo!(
                        "only 'none', 'miter', 'bevel', and 'round' are supported as join types"
                    ),
                }
            }
        }

        return vec![Primitive {
            mesh: Mesh {
                ty: MeshType::Triangle,
                indices: (0..res_vertices.len()).into_iter().collect(),
                vertices: res_vertices,
            },
            material: Material::SingleColor(ColA::WHITE),
        }];
    }
}

pub enum PathSeg {
    CubicBezier {
        P_A: Vec2<Float>,
        P_B: Vec2<Float>,
        P_C: Vec2<Float>,
        P_D: Vec2<Float>,
    },
    QuadraticBezier {
        P_A: Vec2<Float>,
        P_B: Vec2<Float>,
        P_C: Vec2<Float>,
    },
    Conic {
        P_A: Vec2<Float>,
        P_B: Vec2<Float>,
        P_C: Vec2<Float>,
        w_B: Float,
    },
    Line {
        P_A: Vec2<Float>,
        P_B: Vec2<Float>,
    },
}

impl PathSeg {
    pub fn from_elliptical(eq: EllipticalArcEquation) -> Self {
        let P_A = eq.get_pos_from_angle(eq.start_angle);
        let P_C = eq.get_pos_from_angle(Rad::new(eq.start_angle.as_float() + eq.angle_delta));

        let t1 = eq.initial_tangent();
        let t2 = eq.terminal_tangent();

        let P_B = intersect_two_lines(P_A, t1, P_C, t2).unwrap(); // FIXME: handle the exact half circles

        let w_B = (eq.angle_delta / 2.0).cos();

        Self::Conic { P_A, P_B, P_C, w_B }
    }

    fn generator(&self, t: Float) -> Vec2<Float> {
        match self {
            Self::CubicBezier { P_A, P_B, P_C, P_D } => {
                (1.0 - t).powi(3) * *P_A
                    + 3.0 * (1.0 - t).powi(2) * t * *P_B
                    + 3.0 * (1.0 - t) * t.powi(2) * *P_C
                    + t.powi(3) * *P_D
            }
            Self::QuadraticBezier { P_A, P_B, P_C } => {
                (1.0 - t).powi(2) * *P_A + 2.0 * (1.0 - t) * t * *P_B + t.powi(2) * *P_C
            }
            Self::Conic { P_A, P_B, P_C, w_B } => {
                ((1.0 - t).powi(2) * *P_A + 2.0 * (1.0 - t) * t * w_B * *P_B + t.powi(2) * *P_C)
                    / ((1.0 - t).powi(2) + 2.0 * (1.0 - t) * t * w_B + t.powi(2))
            }
            Self::Line { P_A, P_B } => (1.0 - t) * *P_A + t * *P_B,
        }
    }

    fn init_norm_grad(&self) -> Vec2<Float> {
        match self {
            Self::CubicBezier { P_A, P_B, P_C, P_D } => {
                if (*P_B - *P_A).length() > 0.0 {
                    (*P_B - *P_A).normalized()
                } else if (*P_C - *P_A).length() > 0.0 {
                    (*P_C - *P_A).normalized()
                } else {
                    (*P_D - *P_A).normalized()
                }
            }
            Self::QuadraticBezier { P_A, P_B, P_C } => {
                if (*P_B - *P_A).length() > 0.0 {
                    (*P_B - *P_A).normalized()
                } else {
                    (*P_C - *P_A).normalized()
                }
            }
            Self::Conic { P_A, P_B, P_C, w_B } => {
                if (*P_B - *P_A).length() > 0.0 && *w_B != 0.0 {
                    w_B.signum() * (*P_B - *P_A).normalized()
                } else {
                    (*P_C - *P_A).normalized()
                }
            }

            Self::Line { P_A, P_B } => (*P_B - *P_A).normalized(),
        }
    }

    fn term_norm_grad(&self) -> Vec2<Float> {
        match self {
            Self::CubicBezier { P_A, P_B, P_C, P_D } => {
                if (*P_D - *P_C).length() > 0.0 {
                    (*P_D - *P_C).normalized()
                } else if (*P_D - *P_B).length() > 0.0 {
                    (*P_D - *P_B).normalized()
                } else {
                    (*P_D - *P_A).normalized()
                }
            }
            Self::QuadraticBezier { P_A, P_B, P_C } => {
                if (*P_C - *P_B).length() > 0.0 {
                    (*P_C - *P_B).normalized()
                } else {
                    (*P_C - *P_A).normalized()
                }
            }
            Self::Conic { P_A, P_B, P_C, w_B } => {
                if (*P_C - *P_B).length() > 0.0 && *w_B != 0.0 {
                    w_B.signum() * (*P_C - *P_B).normalized()
                } else {
                    (*P_C - *P_A).normalized()
                }
            }

            Self::Line { P_A, P_B } => (*P_B - *P_A).normalized(),
        }
    }

    fn gradient(&self, t: Float) -> Vec2<Float> {
        match *self {
            Self::CubicBezier { P_A, P_B, P_C, P_D } => {
                3.0 * (1.0 - t).powi(2) * (P_B - P_A)
                    + 6.0 * (1.0 - t) * t * (P_C - P_B)
                    + 3.0 * t.powi(2) * (P_D - P_C)
            }
            Self::QuadraticBezier { P_A, P_B, P_C } => {
                2.0 * (1.0 - t) * (P_B - P_A) + 2.0 * t * (P_C - P_B)
            }
            Self::Conic { P_A, P_B, P_C, w_B } => {
                (2.0 * (P_A - P_C) * (-1.0 + t) * t
                    + 2.0
                        * (P_B - P_A * (-1.0 + t).powi(2) - 2.0 * P_B * t + P_C * t.powi(2))
                        * w_B)
                    / (1.0 - 2.0 * (-1.0 + t) * t * (-1.0 + w_B)).powi(2)
            }
            Self::Line { P_A, P_B } => P_B - P_A,
        }
    }

    fn inflections(&self) -> (Vec<Float>, bool) {
        match *self {
            Self::Line { P_A: _, P_B: _ } => (vec![], false),
            Self::QuadraticBezier {
                P_A: _,
                P_B: _,
                P_C: _,
            } => (vec![], false),
            Self::Conic {
                P_A: _,
                P_B: _,
                P_C: _,
                w_B,
            } => {
                let inner = w_B.powi(2) - 1.0;
                let denom = 4.0 * w_B - 4.0;
                if inner < 0.0 || denom == 0. {
                    return (vec![], true);
                }
                let mut res = vec![];
                if inner == 0.0 {
                    if approx_in_range_01(-2.0 / denom) {
                        res.push(-2.0 / denom);
                    }
                } else {
                    let nom_1 = -2.0 + 2.0 * inner.sqrt();
                    let nom_2 = -2.0 - 2.0 * inner.sqrt();

                    if approx_in_range_01(nom_1 / denom) {
                        res.push(nom_1 / denom);
                    }

                    if approx_in_range_01(nom_2 / denom) {
                        res.push(nom_2 / denom);
                    }
                }

                return (res, false);
            }
            Self::CubicBezier { P_A, P_B, P_C, P_D } => {
                // https://www.microsoft.com/en-us/research/wp-content/uploads/2005/01/p1000-loop.pdf
                // power basis -> determinants -> quadratic equation -> roots

                let B = Mat::<4, 3, Float>::new([
                    P_A.expanded_3d().to_arr(),
                    P_B.expanded_3d().to_arr(),
                    P_C.expanded_3d().to_arr(),
                    P_D.expanded_3d().to_arr(),
                ]);

                let M_3 = Mat::new([
                    [1., 0., 0., 0.],
                    [-3., 3., 0., 0.],
                    [3., -6., 3., 0.],
                    [-1., 3., -3., 1.],
                ]);

                let C = M_3 * B;

                let d1 = -Mat::new([
                    [C[3][0], C[3][1], C[3][2]],
                    [C[2][0], C[2][1], C[2][2]],
                    [C[0][0], C[0][1], C[0][2]],
                ])
                .det();

                let d2 = Mat::new([
                    [C[3][0], C[3][1], C[3][2]],
                    [C[1][0], C[1][1], C[1][2]],
                    [C[0][0], C[0][1], C[0][2]],
                ])
                .det();

                let d3 = -Mat::new([
                    [C[2][0], C[2][1], C[2][2]],
                    [C[1][0], C[1][1], C[1][2]],
                    [C[0][0], C[0][1], C[0][2]],
                ])
                .det();

                let res = QuadraticEquation {
                    a: -3. * d1,
                    b: 3. * d2,
                    c: -d3,
                }
                .roots()
                .into_iter()
                .filter(|t| approx_in_range_01(*t))
                .collect::<Vec<_>>();

                if res.len() == 0 {
                    (res, true)
                } else {
                    (res, false)
                }
            }
        }
    }

    fn solve_gradient_n(&self, N: Vec2<Float>) -> Vec<Float> {
        // g'(t) DOT N = 0 not multiplication dumbass
        match *self {
            Self::Line { P_A: _, P_B: _ } => unreachable!(),
            Self::QuadraticBezier { P_A, P_B, P_C } => {
                let t = (N.y * (P_A.y - P_B.y) + P_A.x * N.x - P_B.x * N.x)
                    / (P_A.x * N.x + P_A.y * N.y - 2. * P_B.x * N.x - 2. * P_B.y * N.y
                        + P_C.x * N.x
                        + P_C.y * N.y);

                vec![t]
            }
            Self::CubicBezier { P_A, P_B, P_C, P_D } => {
                let a = 3.0 * ((P_B - P_A) - 2.0 * (P_C - P_B) + (P_D - P_C)).dot(&N);
                let b = 6.0 * ((P_C - P_B) - (P_B - P_A)).dot(&N);
                let c = 3.0 * (P_B - P_A).dot(&N);

                QuadraticEquation { a, b, c }.roots()
            }
            Self::Conic { P_A, P_B, P_C, w_B } => {
                // 2 (A - C) (-1 + t) t + 2w (B - A (-1 + t)^2 - 2 B t + C t^2)
                // 2 (A - C) (-1 + t) t - 2w A (-1 + t)^2 - 4w B t + 2w C t^2 + 2w B
                // alp = 2 (A - C)      bet = 2wA      gam = 4wB  delt = 2w C  eps = 2wB
                // alpha (-1 + t) t - beta (-1 + t)^2 - gamma t + delta t^2 + epsilon
                //
                // A = (α - β + δ)
                // B = - α + 2 β - γ
                // C = ε - β
                let alpha = (2.0 * (P_A - P_C)).dot(&N);
                let beta = (2.0 * w_B * P_A).dot(&N);
                let gamma = (4.0 * w_B * P_B).dot(&N);
                let delta = (2.0 * w_B * P_C).dot(&N);
                let epsilon = (2.0 * w_B * P_B).dot(&N);

                let A = alpha - beta + delta;
                let B = -alpha + 2.0 * beta - gamma;
                let C = epsilon - beta;

                let int = B.powi(2) - 4.0 * A * C;
                let denom = 2.0 * A;
                if int < 0. || denom == 0. {
                    return vec![];
                }

                if int == 0. {
                    let res = (-B) / denom;
                    vec![res]
                } else {
                    let res1 = (-B + int.sqrt()) / denom;
                    let res2 = (-B - int.sqrt()) / denom;

                    vec![res1, res2]
                }
            }
        }
    }

    fn init_n(&self, w: f32) -> Vec2<Float> {
        let r_N = w / 2.0;

        let angle = self.init_norm_grad().x_angle();

        self.generator(0.0) - Vec2::new(-angle.sin(), angle.cos()) * r_N
    }

    fn init_p(&self, w: f32) -> Vec2<Float> {
        let r_P = w / 2.0;

        let angle = self.init_norm_grad().x_angle();

        self.generator(0.0) + Vec2::new(-angle.sin(), angle.cos()) * r_P
    }

    fn term_n(&self, w: f32) -> Vec2<Float> {
        let r_N = w / 2.0;

        let angle = self.term_norm_grad().x_angle();

        self.generator(1.0) - Vec2::new(-angle.sin(), angle.cos()) * r_N
    }

    fn term_p(&self, w: f32) -> Vec2<Float> {
        let r_P = w / 2.0;

        let angle = self.term_norm_grad().x_angle();

        self.generator(1.0) + Vec2::new(-angle.sin(), angle.cos()) * r_P
    }
}

pub fn stroke(seg: &PathSeg, w: Float) -> Vec<(Vec2<Float>, Vec2<Float>)> {
    let q = QUALITY;

    let (mut inflection_params, more_needed) = seg.inflections();

    let mut p_i = vec![0.0];
    if more_needed {
        p_i.push(0.5);
    } else {
        p_i.append(&mut inflection_params);
    }
    p_i.push(1.0);

    let n = p_i.len();

    // this is changed from how it works in the paper bc. the split function did not work
    // might have to revisit it in the future for real test cases where the split shouldnt
    // be right in the middle
    let cpsi_i = (0..=n)
        .map(|i| {
            if i == 0 {
                seg.init_norm_grad().x_angle()
            } else if i == n {
                seg.term_norm_grad().x_angle()
            } else {
                seg.gradient(p_i[i]).x_angle()
            }
        })
        .collect::<Vec<_>>();

    let M = p_i.len() - 1;

    let delta_i = (1..M + 1)
        .map(|i| rel_angle_diff(cpsi_i[i], cpsi_i[i - 1])) // this is wrong in paper they are flipped
        .collect::<Vec<_>>();

    let cdelta_i = |i: usize| ((delta_i[i].abs() / q).ceil() as usize).clamp(1, usize::MAX);

    let cdelta_sum_of_k = (0..=M)
        .map(|k| (0..k).map(|i| cdelta_i(i)).sum())
        .collect::<Vec<_>>();

    let t_of_range_psi = |k: usize, psi: f32| {
        let N = Vec2::new(-psi.sin(), psi.cos());

        let solved = seg.solve_gradient_n(N);

        let range = p_i[k]..p_i[k + 1];

        solved
            .into_iter()
            .find(|s| range.contains(s))
            .unwrap_or_else(|| {
                if (seg.gradient(p_i[k]).dot(&N)) < (seg.gradient(p_i[k + 1]).dot(&N)) {
                    p_i[k]
                } else {
                    p_i[k + 1]
                }
            })
    };

    let psi_of_j = |j: usize| match find_j_equal_or_between(j, &cdelta_sum_of_k) {
        EqBetw::Equal(k) => cpsi_i[k],
        EqBetw::Between(k) => {
            cpsi_i[k] + (delta_i[k] / cdelta_i(k) as f32) * (j - cdelta_sum_of_k[k]) as f32
        }
    };

    let t_of_j = |j: usize| match find_j_equal_or_between(j, &cdelta_sum_of_k) {
        EqBetw::Equal(k) => p_i[k],
        EqBetw::Between(k) => t_of_range_psi(k, psi_of_j(j)),
    };

    let n_of_j = |j: usize| match find_j_equal_or_between(j, &cdelta_sum_of_k) {
        EqBetw::Equal(k) => Vec2::new(-cpsi_i[k].sin(), cpsi_i[k].cos()),
        EqBetw::Between(_) => Vec2::new(-psi_of_j(j).sin(), psi_of_j(j).cos()),
    };

    let N = cdelta_sum_of_k[M];

    let r_N = w / 2.0;
    let r_P = w / 2.0;

    let N_j = |j: usize| seg.generator(t_of_j(j)) - r_N * n_of_j(j);
    let P_j = |j: usize| seg.generator(t_of_j(j)) + r_P * n_of_j(j);

    let mut res = vec![];
    for j in 0..=N {
        let v1 = N_j(j);
        let v2 = P_j(j);
        res.push((v1, v2));
    }

    res
}

enum EqBetw {
    Equal(usize),
    Between(usize),
}

fn find_j_equal_or_between(j: usize, arr: &Vec<usize>) -> EqBetw {
    let mut k = 0;

    while arr[k] < j {
        k += 1;
    }

    if arr[k] == j {
        return EqBetw::Equal(k);
    } else {
        return EqBetw::Between(k - 1);
    }
}

fn rel_angle_diff(theta_1: f32, theta_2: f32) -> f32 {
    if (theta_1 - theta_2) > (PI) {
        theta_1 - theta_2 - PI2
    } else if (theta_1 - theta_2) < (-PI) {
        theta_1 - theta_2 + PI2
    } else {
        theta_1 - theta_2
    }
}

// fn angle_addition(theta_1: f32, theta_2: f32) -> f32 {
//     if (theta_1 + theta_2) > PI {
//         theta_1 + theta_2 - PI2
//     } else if (theta_1 + theta_2) < (-PI) {
//         theta_1 + theta_2 + PI2
//     } else {
//         theta_1 + theta_2
//     }
// }
