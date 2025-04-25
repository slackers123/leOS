use corelib::types::Float;
use mathlib::{aabb::AABB, color::ColA, elliptical_arc::EllipticalArc, vectors::Vec2};

pub mod path_drawable;

pub struct Path {
    pub pos: Vec2<Float>,
    path_segs: Vec<PathSeg>,
    vals: Vec<Float>,
    bbox: AABB<Float>,
    col: ColA,
}

impl Path {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            path_segs: Vec::new(),
            vals: Vec::new(),
            bbox: AABB::default(),
            col: ColA::WHITE,
        }
    }

    pub fn move_to(&mut self, target: Vec2<Float>) {
        self.bbox.include_vec(&target);
        self.path_segs.push(PathSeg::MoveTo);
        self.vals.push(target.x);
        self.vals.push(target.y);
    }

    pub fn line_to(&mut self, target: Vec2<Float>) {
        self.bbox.include_vec(&target);
        self.path_segs.push(PathSeg::LineTo);
        self.vals.push(target.x);
        self.vals.push(target.y);
    }

    pub fn hline_to(&mut self, x: Float) {
        let last = self.vals.len() - 1;

        self.line_to(Vec2::new(x, self.vals[last]));
    }

    pub fn vline_to(&mut self, y: Float) {
        let last = self.vals.len() - 2;

        self.line_to(Vec2::new(self.vals[last], y));
    }

    pub fn q_bezier_to(&mut self, control: Vec2<Float>, target: Vec2<Float>) {
        self.bbox.include_vec(&control);
        self.bbox.include_vec(&target);
        self.path_segs.push(PathSeg::QBezierTo);
        self.vals.push(control.x);
        self.vals.push(control.y);
        self.vals.push(target.x);
        self.vals.push(target.y);
    }

    pub fn c_bezier_to(
        &mut self,
        control1: Vec2<Float>,
        control2: Vec2<Float>,
        target: Vec2<Float>,
    ) {
        self.bbox.include_vec(&control1);
        self.bbox.include_vec(&control2);
        self.bbox.include_vec(&target);
        self.path_segs.push(PathSeg::CBezierTo);
        self.vals.push(control1.x);
        self.vals.push(control1.y);
        self.vals.push(control2.x);
        self.vals.push(control2.y);
        self.vals.push(target.x);
        self.vals.push(target.y);
    }

    pub fn arc_to(
        &mut self,
        radii: Vec2<Float>,
        x_axis_rotation: Float,
        large_arc_flag: Float,
        sweep_flag: Float,
        target: Vec2<Float>,
    ) {
        self.bbox.include_vec(&target);
        self.path_segs.push(PathSeg::ArcTo);
        self.vals.push(radii.x);
        self.vals.push(radii.y);
        self.vals.push(x_axis_rotation);
        self.vals.push(large_arc_flag);
        self.vals.push(sweep_flag);
        self.vals.push(target.x);
        self.vals.push(target.y);
    }

    pub fn close_path(&mut self) {
        self.path_segs.push(PathSeg::ClosePath);
    }

    pub fn points_iter<'a>(&'a self) -> PathPointsIter<'a> {
        PathPointsIter {
            val_index: 0,
            seg_idx: 0,
            path: &self,
        }
    }

    pub fn segs_iter<'a>(&'a self) -> PathSegIter<'a> {
        PathSegIter {
            val_index: 0,
            seg_idx: 0,
            path: &self,
        }
    }

    pub fn to_drawable(&self) -> crate::stroking::Path {
        let mut last = Vec2::ZERO;
        let segs = self
            .segs_iter()
            .flat_map(|s| {
                let res = match s {
                    CompletePathSeg::MoveTo(t) => {
                        last = t;
                        None
                    }
                    CompletePathSeg::LineTo(t) => {
                        Some(crate::stroking::PathSeg::Line { P_A: last, P_B: t })
                    }

                    CompletePathSeg::ClosePath => {
                        None // TODO: actually do something
                    }
                    CompletePathSeg::CBezierTo(b, c, d) => {
                        Some(crate::stroking::PathSeg::CubicBezier {
                            P_A: last,
                            P_B: b,
                            P_C: c,
                            P_D: d,
                        })
                    }
                    CompletePathSeg::QBezierTo(b, c) => {
                        Some(crate::stroking::PathSeg::QuadraticBezier {
                            P_A: last,
                            P_B: b,
                            P_C: c,
                        })
                    }
                    CompletePathSeg::ArcTo(r, rot, large_arc_flag, sweep_flag, end) => {
                        Some(crate::stroking::PathSeg::from_elliptical(
                            EllipticalArc {
                                start: last,
                                r,
                                rot,
                                large_arc_flag: large_arc_flag == 1.0, // FIXME: this is not exhaustive
                                sweep_flag: sweep_flag == 1.0,
                                end,
                            }
                            .to_equation(),
                        ))
                    }
                };
                if res.is_some() {
                    last = s.get_target();
                }

                res
            })
            .collect::<Vec<_>>();

        crate::stroking::Path {
            // TODO: make these actual parameters
            segs,
            join_type: crate::stroking::JoinType::None,
            width: 10.0,
        }
    }
}

pub enum PathSeg {
    MoveTo,
    LineTo,
    QBezierTo,
    CBezierTo,
    ArcTo,
    ClosePath,
}

impl PathSeg {
    pub fn num_points(&self) -> u8 {
        use PathSeg::*;
        match self {
            MoveTo => 2,
            LineTo => 2,
            QBezierTo => 4,
            CBezierTo => 6,
            ArcTo => 7,
            ClosePath => 0,
        }
    }
}

pub struct PathPointsIter<'a> {
    val_index: usize,
    seg_idx: usize,
    path: &'a Path,
}

impl<'a> Iterator for PathPointsIter<'a> {
    type Item = Vec<Vec2<Float>>;
    fn next(&mut self) -> Option<Self::Item> {
        let seg = self.path.path_segs.get(self.seg_idx)?;
        let point;
        match seg {
            PathSeg::MoveTo => {
                let p = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                point = Some(vec![p])
            }
            PathSeg::LineTo => {
                let p = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                point = Some(vec![p])
            }
            PathSeg::QBezierTo => {
                let c = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                let p = Vec2::new(
                    self.path.vals[self.val_index + 2],
                    self.path.vals[self.val_index + 3],
                );
                point = Some(vec![c, p])
            }
            PathSeg::CBezierTo => {
                let c1 = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                let c2 = Vec2::new(
                    self.path.vals[self.val_index + 2],
                    self.path.vals[self.val_index + 3],
                );
                let p = Vec2::new(
                    self.path.vals[self.val_index + 4],
                    self.path.vals[self.val_index + 5],
                );
                point = Some(vec![c1, c2, p])
            }
            PathSeg::ArcTo => {
                let p = Vec2::new(
                    self.path.vals[self.val_index + 5],
                    self.path.vals[self.val_index + 6],
                );
                point = Some(vec![p])
            }
            PathSeg::ClosePath => point = Some(vec![]),
        }
        self.val_index += seg.num_points() as usize;
        self.seg_idx += 1;
        point
    }
}

#[derive(Debug, Clone)]
pub enum CompletePathSeg {
    MoveTo(Vec2<Float>),
    LineTo(Vec2<Float>),
    QBezierTo(Vec2<Float>, Vec2<Float>),
    CBezierTo(Vec2<Float>, Vec2<Float>, Vec2<Float>),
    ArcTo(Vec2<Float>, Float, Float, Float, Vec2<Float>),
    ClosePath,
}

impl CompletePathSeg {
    pub fn get_target(&self) -> Vec2<Float> {
        match self {
            Self::MoveTo(t) => *t,
            Self::LineTo(t) => *t,
            Self::QBezierTo(_, t) => *t,
            Self::CBezierTo(_, _, t) => *t,
            Self::ArcTo(_, _, _, _, t) => *t,
            Self::ClosePath => Vec2::ZERO,
        }
    }
}

pub struct PathSegIter<'a> {
    val_index: usize,
    seg_idx: usize,
    path: &'a Path,
}

impl<'a> Iterator for PathSegIter<'a> {
    type Item = CompletePathSeg;
    fn next(&mut self) -> Option<Self::Item> {
        let seg = self.path.path_segs.get(self.seg_idx)?;
        let complete_path_seg;
        match seg {
            PathSeg::MoveTo => {
                let p = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                complete_path_seg = CompletePathSeg::MoveTo(p)
            }
            PathSeg::LineTo => {
                let p = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                complete_path_seg = CompletePathSeg::LineTo(p)
            }
            PathSeg::QBezierTo => {
                let c = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                let p = Vec2::new(
                    self.path.vals[self.val_index + 2],
                    self.path.vals[self.val_index + 3],
                );
                complete_path_seg = CompletePathSeg::QBezierTo(c, p)
            }
            PathSeg::CBezierTo => {
                let c1 = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                let c2 = Vec2::new(
                    self.path.vals[self.val_index + 2],
                    self.path.vals[self.val_index + 3],
                );
                let p = Vec2::new(
                    self.path.vals[self.val_index + 4],
                    self.path.vals[self.val_index + 5],
                );
                complete_path_seg = CompletePathSeg::CBezierTo(c1, c2, p)
            }
            PathSeg::ArcTo => {
                let r = Vec2::new(
                    self.path.vals[self.val_index + 0],
                    self.path.vals[self.val_index + 1],
                );
                let x_axis_rotation = self.path.vals[self.val_index + 2];
                let large_arc_flag = self.path.vals[self.val_index + 3];
                let sweep_flag = self.path.vals[self.val_index + 4];
                let p = Vec2::new(
                    self.path.vals[self.val_index + 5],
                    self.path.vals[self.val_index + 6],
                );
                complete_path_seg =
                    CompletePathSeg::ArcTo(r, x_axis_rotation, large_arc_flag, sweep_flag, p)
            }
            PathSeg::ClosePath => complete_path_seg = CompletePathSeg::ClosePath,
        }
        self.val_index += seg.num_points() as usize;
        self.seg_idx += 1;
        Some(complete_path_seg)
    }
}
