// mod imgsave;
// mod parsertest;

use std::{fs, ptr::read};

use dpilib::LUnit;
use drawlib::{
    path_attr::{PathAttrs, PathFill, PathStroke},
    shape_primitive::{line::Line, rect::Rect},
    stroking::JoinType,
};
use imglib::{RgbaF32, RgbaF32Image, RgbaU8, RgbaU8Image};
use mathlib::{bezier::CubicBezier, color::ColA, elliptical_arc::EllipticalArc, vectors::Vec2};

// use crate::imgsave::Qimg;

fn main() {
    // let file = fs::read("../test-data/qoi_test_images/wikipedia_008.qoi").unwrap();

    // let mut reader = corelib::reader::Reader::new_big_endian(&file, 0);

    // let reader = qoilib::reader::QoiReader::new(&mut reader);

    // let (header, img) = reader.read_entire_image();

    // let mut file = fs::File::create("test.qoi").unwrap();

    // let mut writer = qoilib::QoiWriter::new(header, &img, &mut file);

    // writer.write();

    // let shapes = guilib::gui_test();
    // let primitives = drawlib::tesselate(&shapes);

    let rect = Rect::new(
        10.,
        10.,
        200.,
        200.,
        0.,
        0.,
        PathAttrs {
            stroke: PathStroke {
                color: ColA::PINK,
                width: LUnit::new(20.0),
                join: JoinType::Miter,
            },
            fill: PathFill { color: ColA::GREEN },
        },
    );

    let path = rect.to_path();

    println!("{path:?}");

    let primitives = drawlib::tesselate(&[path]);

    let mut image = RgbaF32Image::new(1000, 1000, RgbaF32::TRANSPARENT);
    renderlib::draw_primitives(&primitives, &mut image);
    image.to_u8().save("test.qoi").unwrap();
    println!("asdf");

    // img.save("out.png").unwrap();

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
