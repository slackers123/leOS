use corelib::types::{Float, Uint};
use mathlib::vectors::Vec2;

use crate::{drawable::Drawable, path::CompletePathSeg, rendererror::RenderResult};

use super::Path;

impl Drawable for Path {
    fn draw(&self, target: &mut impl crate::draw_target::DrawTarget) -> RenderResult<()> {
        println!("{:?}", self.bbox);
        for y in self.bbox.min.y as usize..self.bbox.max.y as usize {
            for x in self.bbox.min.x as usize..self.bbox.max.x as usize {
                let mut count = 0;
                let segs = self.segs_iter();
                let mut first = None;
                let mut last = Vec2::ZERO;
                let mut initial_point = Vec2::ZERO;
                for seg in segs {
                    if first.is_none() {
                        first = Some(seg.clone());
                    }
                    count += match seg {
                        CompletePathSeg::MoveTo(t) => {
                            initial_point = t;
                            0
                        }
                        CompletePathSeg::LineTo(t) => {
                            line_isect(t, last, Vec2::new(x as Float, y as Float))
                        }
                        CompletePathSeg::QBezierTo(p1, p2) => {
                            qbezier_isect(last, p1, p2, Vec2::new(x as Float, y as Float))
                        }
                        CompletePathSeg::ClosePath => {
                            line_isect(initial_point, last, Vec2::new(x as Float, y as Float))
                        }
                        _ => todo!("implement"),
                    };
                    last = seg.get_target();
                }
                if count % 2 == 1 {
                    target.put_pixel(Vec2::new(x as Uint, y as Uint), self.col)?;
                }
            }
        }
        Ok(())
    }
}

fn qbezier_isect(p0: Vec2<Float>, p1: Vec2<Float>, p2: Vec2<Float>, p: Vec2<Float>) -> Uint {
    let b = p.y;
    let y_0 = p0.y;
    let y_1 = p1.y;
    let y_2 = p2.y;
    let root_term = -2.0 * b * y_1 + y_0 * (b - y_2) + b * y_2 + y_1 * y_1;
    if root_term < 0.0 {
        return 0;
    }
    let denom = y_0 - 2.0 * y_1 + y_2;
    if denom == 0.0 {
        return 0;
    }
    let root_term_sqrt = root_term.sqrt();
    let t = -(root_term_sqrt - y_0 + y_1) / (denom);
    let t1 = (root_term_sqrt + y_0 - y_1) / (denom);
    let mut count = if t > 0.0 && t < 1.0 {
        let x = (1.0 - t) * (1.0 - t) * p0.x + 2.0 * (1.0 - t) * t * p1.x + t * t * p2.x;
        if x < p.x {
            1
        } else {
            0
        }
    } else {
        0
    };

    count += if t1 > 0.0 && t1 < 1.0 {
        let x1 = (1.0 - t1) * (1.0 - t1) * p0.x + 2.0 * (1.0 - t1) * t1 * p1.x + t1 * t1 * p2.x;
        if x1 < p.x {
            1
        } else {
            0
        }
    } else {
        0
    };
    count
}

fn line_isect(t: Vec2<Float>, last_point: Vec2<Float>, p: Vec2<Float>) -> Uint {
    if p.y > t.y && p.y > last_point.y {
        return 0;
    }
    if p.y < t.y && p.y < last_point.y {
        return 0;
    }
    let d = last_point - t;

    let k = d.y / d.x;
    let x_on_line = (p.y - t.y) / k + t.x;

    if x_on_line < t.x && x_on_line < last_point.x {
        return 0;
    }

    if x_on_line > t.x && x_on_line > last_point.x {
        return 0;
    }

    if x_on_line > p.x {
        return 0;
    }

    1
}
