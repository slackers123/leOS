use corelib::types::{Float, Uint};
use drawlib::{
    draw_target::DrawTarget,
    drawable::Drawable,
    path::{path_drawable::isect_at, CompletePathSeg, Path},
    text::get_char_path,
};
use imgsave::Qimg;
use mathlib::{color::ColA, vectors::Vec2};

mod imgsave;

fn main() {
    let source = std::fs::read("../test-data/Roboto-Regular.ttf").unwrap();
    let font = ttflib::load_ttf(&source);

    let src = "asdf";
    let mut img = Qimg(image::ImageBuffer::new(1000, 1000));

    for (i, c) in src.chars().enumerate() {
        let mut path = get_char_path(c, &font);
        path.pos.x = i as Float * 200.0;

        path.draw(&mut img).unwrap();
    }

    // println!("{a_glyf:?}");

    // println!("font: {:?}", font.get_glyph('a'));

    img.0.save("out.png").unwrap();

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
}
