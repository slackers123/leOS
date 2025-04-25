use std::io::Lines;

use drawlib::{
    drawable::Drawable,
    primitives::{circle::Circle, ellipse::Ellipse, line::Line, rect::Rect},
    stroking::{JoinType, Path, PathSeg},
};
use imgsave::Qimg;
use mathlib::vectors::Vec2;

mod imgsave;
// mod parsertest;

fn main() {
    // let toks =
    //     htmllib::tokenize(std::fs::read_to_string("sample-html-files-sample1.html").unwrap());
    // println!("{toks:?}");

    // let regex = regexlib::Regex::new(r"a{4,}");
    // assert!(!regex.validate(""));
    // assert!(!regex.validate("a"));
    // assert!(!regex.validate("aa"));
    // assert!(regex.validate("aaa"));
    // assert!(regex.validate("aaaa"));
    // assert!(!regex.validate("aaaaa"));

    // let source = std::fs::read("../test-data/Roboto-Regular.ttf").unwrap();
    // let font = ttflib::load_ttf(&source);

    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 1000;
    // const WIDTH2: u32 = WIDTH / 2;
    // const HEIGHT2: u32 = HEIGHT / 2;

    let mut img = Qimg(image::ImageBuffer::new(WIDTH, HEIGHT));

    let rect = Line::new(500.0, 500.0, 100.0, 50.0);

    rect.draw(&mut img).unwrap();

    // let q = CubicBezier::new(
    //     Vec2::new(10., 10.),
    //     Vec2::new(10., 90.),
    //     Vec2::new(90., 10.),
    //     Vec2::new(90., 90.),
    // );

    // let arc = EllipticalArc {
    //     r: Vec2::new(10., 20.),
    //     rot: 0.,
    //     start: Vec2::new(40., 50.),
    //     end: Vec2::new(60., 50.),
    //     large_arc_flag: false,
    //     sweep_flag: false,
    // }
    // .to_equation();

    // println!("{arc:?}");

    // let arc = EllipticalArcEquation {
    //     r: Vec2::new(10., 20.),
    //     rot: 0.,
    //     c: Vec2::new(50.0, 50.0),
    //     start_angle: 0.,
    //     angle_delta: PI,
    // };

    // let l1 = LineSegment::new(Vec2::new(0., 10.), Vec2::new(90., 100.));
    // let l2 = LineSegment::new(Vec2::new(10., 0.), Vec2::new(100., 90.));

    // let l3 = LineSegment::new(Vec2::new(0., 10.), Vec2::new(10., 0.));
    // let l4 = LineSegment::new(Vec2::new(100., 90.), Vec2::new(90., 100.));

    // let segs = [l1, l2, l3, l4];

    // let bboxes: Vec<AABB<Float>> = segs.iter().map(|s| s.bbox()).collect();

    // for y in 0..HEIGHT {
    //     let roots: Vec<Float> = segs
    //         .iter()
    //         .zip(bboxes.iter())
    //         .filter(|s| s.1.y_inside(y as Float))
    //         .map(|s| s.0.isect_at_y(y as Float))
    //         .flatten()
    //         .collect();

    //     for x in 0..WIDTH {
    //         let isects = roots.iter().filter(|r| **r < x as f32).count();

    //         match isects {
    //             0 => img.0.put_pixel(x, y, image::Rgba([0, 0, 0, 255])),
    //             1 => img.0.put_pixel(x, y, image::Rgba([0, 255, 0, 255])),
    //             2 => img.0.put_pixel(x, y, image::Rgba([0, 0, 255, 255])),
    //             _ => img.0.put_pixel(x, y, image::Rgba([255, 0, 0, 255])),
    //         }
    //     }
    // }

    img.0.save("out.png").unwrap();

    // let mut p = Path::new();
    // p.move_to(Vec2::new(0., 0.));
    // p.c_bezier_to(
    //     Vec2::new(0., 100.),
    //     Vec2::new(100., 0.),
    //     Vec2::new(100., 100.),
    // );
    // // p.line_to(Vec2::new(10., 10.));
    // p.close_path();

    // p.draw(&mut img).unwrap();

    // img.0.save("out.png").unwrap();

    // println!(
    //     "{}",
    //     cbezier_isects(
    //         CubicBezier::new(
    //             Vec2::new(0., 0.),
    //             Vec2::new(0., 100.),
    //             Vec2::new(100., 0.),
    //             Vec2::new(100., 100.),
    //         ),
    //         Vec2::new(20., 49.),
    //     )
    // );

    // for (i, c) in src.chars().enumerate() {
    //     let mut path = get_char_path(c, &font);
    //     path.pos.x = i as Float * 250.0;

    //     path.draw(&mut img).unwrap();
    // }

    // println!("{a_glyf:?}");

    // println!("font: {:?}", font.get_glyph('a'));

    // let rect = Rect2::new(
    //     Vec2::splat(100.0),
    //     Vec2::splat(50.0),
    //     Vec2::splat(20.0),
    //     Stroke {
    //         thickness: 10.0,
    //         col: ColA::WHITE,
    //     },
    //     Fill {},
    // );

    // rect.draw(&mut img);

    // let line = Line2::new(
    //     Vec2::splat(10.0),
    //     Vec2::splat(100.0),
    //     Stroke {
    //         thickness: 10.0,
    //         col: ColA::WHITE,
    //     },
    //     Fill {},
    // );

    // line.draw(&mut img);

    // let quad = ConvexQuad::new(
    //     Vec2::new(20.0, 20.0),
    //     Vec2::new(25.0, 30.0),
    //     Vec2::new(20.0, 40.0),
    //     Vec2::new(10.0, 30.0),
    //     ColA::WHITE,
    // );

    // quad.draw(&mut img);

    // renderer.add_renderable(Renderable::Rect2(rect));

    // for y in 0..100 {
    //     println!("{}", isect_at(&p, Vec2::new(100., y as Float)));
    // }
}

// fn isect(arc: &EllipticalArc, test_point: Vec2<Float>) -> Uint {
//     // let bbox = arc.bbox();
//     // if !test_point_in_range(bbox.min.y, bbox.max.y, test_point) {
//     //     return 0;
//     // }

//     let moved = arc.moved_y(-test_point.y);

//     let equation = moved.get_equation();

//     // println!("{equation:?}");

//     let roots = equation.roots();

//     println!("{roots:?}");

//     let roots = roots
//         .into_iter()
//         .filter(|t| {
//             // println!("filter: {t}");
//             arc.root_filter(t, test_point.x)
//         })
//         .count();

//     return roots as Uint;
// }
