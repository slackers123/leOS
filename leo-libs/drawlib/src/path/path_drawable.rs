use crate::{drawable::Drawable, rendererror::RenderResult};

use super::Path;

impl Drawable for Path {
    fn draw(&self, target: &mut impl crate::draw_target::DrawTarget) -> RenderResult<()> {
        // TODO: path filling here
        self.to_drawable().draw(target)
    }
}

// pub fn isect_at(path: &Path, p: Vec2<Float>) -> Uint {
//     let mut count = 0;
//     let segs = path.segs_iter();
//     let mut first = None;
//     let mut last = Vec2::ZERO;
//     let mut initial_point = Vec2::ZERO;
//     for seg in segs {
//         if first.is_none() {
//             first = Some(seg.clone());
//         }
//         count += match seg {
//             CompletePathSeg::MoveTo(t) => {
//                 initial_point = t;
//                 0
//             }
//             CompletePathSeg::LineTo(t) => line_isects(last, t, p),
//             CompletePathSeg::QBezierTo(p1, p2) => qbezier_isects(QuadraticBezier, p, Vec2::ZERO),
//             CompletePathSeg::ClosePath => line_isects(initial_point, last, p),
//             _ => todo!("implement"),
//         };
//         last = seg.get_target();
//     }
//     count
// }

// pub fn arc_isects(mut arc: EllipticalArc, test_point: Vec2<Float>) -> Uint {}

// pub fn cbezier_isects(mut bezier: CubicBezier, test_point: Vec2<Float>) -> Uint {
//     let bezier_bb = bezier.bbox();
//     if !test_point_in_range(bezier_bb.min.y, bezier_bb.max.y, test_point) {
//         return 0;
//     }

//     bezier.move_y(-test_point.y);
//     let CubicBezier { p0, p1, p2, p3 } = bezier;

//     let a = -p0.y + 3. * p1.y - 3. * p2.y + p3.y;
//     let b = 3. * p0.y - 6. * p1.y + 3. * p2.y;
//     let c = -3. * p0.y + 3. * p1.y;
//     let d = p0.y;

//     let equation = CubicEquation { a, b, c, d };

//     let roots = equation
//         .roots()
//         .into_iter()
//         .filter(|t| approx_in_range_01(t) && bezier.pos_from_t(*t).x < test_point.x)
//         .collect::<Vec<Float>>();

//     return roots.len() as Uint;
// }

// pub fn qbezier_isects(mut bezier: QuadraticBezier, test_point: Vec2<Float>) -> Uint {
//     let bezier_bb = bezier.bbox();
//     if !test_point_in_range(bezier_bb.min.y, bezier_bb.max.y, test_point) {
//         return 0;
//     }

//     bezier.move_y(-test_point.y);

//     let a = bezier.p0.y - 2. * bezier.p1.y + bezier.p2.y;
//     let b = 2. * (bezier.p1.y - bezier.p0.y);
//     let c = bezier.p0.y;

//     let equation = QuadraticEquation { a, b, c };

//     // println!("{equation:?}");

//     let roots = equation
//         .roots()
//         .into_iter()
//         .filter(|t| approx_in_range_01(t) && bezier.pos_from_t(*t).x < test_point.x)
//         .collect::<Vec<Float>>();

//     // println!(
//     //     "{:?} {:?}",
//     //     equation.y_from_x(roots[0]),
//     //     equation.y_from_x(roots[1])
//     // );

//     return roots.len() as Uint;
// }

// fn line_isects(mut start: Vec2<Float>, mut end: Vec2<Float>, test_point: Vec2<Float>) -> Uint {
//     // println!("{test_point:?}");
//     if !test_point_in_range(start.y.min(end.y), start.y.max(end.y), test_point) {
//         return 0;
//     }
//     start.y -= test_point.y;
//     end.y -= test_point.y;

//     let dx = end.x - start.x;
//     let dy = end.y - start.y;
//     let k = dx / dy;
//     let d = start.y - k * start.x;

//     let roots = LinearEquation { a: k, b: d }.roots();
//     if !roots.is_empty() {
//         let root = -roots[0];
//         if root < test_point.x {
//             return 1;
//         }
//     }
//     return 0;
// }

// fn test_point_in_range(y_min: Float, y_max: Float, test_point: Vec2<Float>) -> bool {
//     approx_in_range(&test_point.y, y_min, y_max)
// }
