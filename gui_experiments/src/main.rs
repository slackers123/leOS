// use drawlib::{drawable::Drawable, path::Path};
// use imgsave::Qimg;
// use mathlib::vectors::Vec2;

use std::f32::consts::PI;

use mathlib::{elliptical_arc::EllipticalArc, vectors::Vec2};

mod imgsave;
// mod parsertest;

fn main() {
    // let toks =
    //     htmllib::tokenize(std::fs::read_to_string("sample-html-files-sample1.html").unwrap());
    // println!("{toks:?}");

    let regex = regexlib::new_regex(r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9.-]+");
    let res = regexlib::validate_regex(&regex, "severin.gebesmair@proton.me");
    assert!(res);

    // let source = std::fs::read("../test-data/Roboto-Regular.ttf").unwrap();
    // let font = ttflib::load_ttf(&source);

    // let src = "asdf";
    // let mut img = Qimg(image::ImageBuffer::new(100, 100));

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
