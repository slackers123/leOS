use corelib::types::{Float, Uint};
use mathlib::{color::ColA, vectors::Vec2};

use crate::{
    drawable::Drawable,
    primitive::{Material, Mesh, Primitve},
};

/// A pixel triangle in screen space.
///
/// It is used internally to render almost everything
pub struct PTri {
    pub a: Vec2<Float>,
    pub b: Vec2<Float>,
    pub c: Vec2<Float>,
}

impl PTri {
    pub fn new(a: Vec2<Float>, b: Vec2<Float>, c: Vec2<Float>) -> Self {
        Self { a, b, c }
    }
}

impl Drawable for PTri {
    fn to_primitives(self) -> Vec<crate::primitive::Primitve> {
        vec![Primitve {
            mesh: Mesh {
                vertices: vec![self.a, self.b, self.c],
                indices: vec![0, 1, 2],
            },
            material: Material::SingleColor(ColA::WHITE),
        }]
    }

    // fn draw(&self, target: &mut impl crate::draw_target::DrawTarget) {
    //     // Ensure the triangle vertices are in clockwise order
    //     let cross_product = (self.b.x - self.a.x) * (self.c.y - self.a.y)
    //         - (self.b.y - self.a.y) * (self.c.x - self.a.x);

    //     let tria = self.a;
    //     let mut trib = self.b;
    //     let mut tric = self.c;

    //     if cross_product < 0.0 {
    //         // Swap b and c to make the order clockwise
    //         let temp = trib;
    //         trib = tric;
    //         tric = temp;
    //     }

    //     let min_x = tria.x.min(trib.x).min(tric.x);
    //     let max_x = tria.x.max(trib.x).max(tric.x);
    //     let min_y = tria.y.min(trib.y).min(tric.y);
    //     let max_y = tria.y.max(trib.y).max(tric.y);

    //     let dims = target.dimensions();

    //     let min_x = min_x.clamp(0.0, dims.0 as Float);
    //     let max_x = max_x.clamp(0.0, dims.0 as Float);
    //     let min_y = min_y.clamp(0.0, dims.1 as Float);
    //     let max_y = max_y.clamp(0.0, dims.1 as Float);

    //     for y in min_y as Uint..=max_y as Uint {
    //         for x in min_x as Uint..=max_x as Uint {
    //             let p = Vec2::new(x as Float, y as Float);

    //             let edge1 = (trib.x - tria.x) * (p.y - tria.y) - (trib.y - tria.y) * (p.x - tria.x);
    //             let edge2 = (tric.x - trib.x) * (p.y - trib.y) - (tric.y - trib.y) * (p.x - trib.x);
    //             let edge3 = (tria.x - tric.x) * (p.y - tric.y) - (tria.y - tric.y) * (p.x - tric.x);

    //             if edge1 >= 0.0 && edge2 >= 0.0 && edge3 >= 0.0 {
    //                 target.put_pixel(Vec2::new(x, y), ColA::BLUE);
    //             }
    //         }
    //     }
    // }
}
