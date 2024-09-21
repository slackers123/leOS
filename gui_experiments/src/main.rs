use corelib::types::{Float, Uint};
use drawlib::{
    draw_target::DrawTarget,
    drawable::Drawable,
    path::{CompletePathSeg, Path},
};
use imgsave::Qimg;
use mathlib::{color::ColA, vectors::Vec2};

mod imgsave;

fn main() {
    // let source = std::fs::read("../test-data/Roboto-Regular.ttf").unwrap();
    // let mut font = ttflib::load_ttf(&source);

    // println!("font: {:?}", font.get_glyph('a'));

    let mut img = Qimg(image::ImageBuffer::new(200, 200));

    let mut path = Path::new();

    path.move_to(Vec2::new(0.0, 0.0));
    path.q_bezier_to(Vec2::new(50.0, 100.0), Vec2::new(100.0, 0.0));
    path.line_to(Vec2::new(150.0, 100.0));
    path.line_to(Vec2::new(100.0, 150.0));
    path.close_path();

    path.draw(&mut img).unwrap();

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

    img.0.save("out.png").unwrap();
    // renderer.add_renderable(Renderable::Rect2(rect));
}
