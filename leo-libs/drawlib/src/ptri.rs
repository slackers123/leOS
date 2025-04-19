use corelib::types::Uint;
use mathlib::{color::ColA, vectors::Vec2};

use crate::drawable::Drawable;

/// A pixel triangle in screen space.
///
/// It is used internally to render almost everything
pub struct PTri {
    pub a: Vec2<Uint>,
    pub b: Vec2<Uint>,
    pub c: Vec2<Uint>,
}

impl PTri {
    pub fn new(a: Vec2<Uint>, b: Vec2<Uint>, c: Vec2<Uint>) -> Self {
        Self { a, b, c }
    }
}

impl Drawable for PTri {
    fn draw(
        &self,
        target: &mut impl crate::draw_target::DrawTarget,
    ) -> crate::rendererror::RenderResult<()> {
        // Ensure the triangle vertices are in clockwise order
        let cross_product = (self.b.x as i32 - self.a.x as i32)
            * (self.c.y as i32 - self.a.y as i32)
            - (self.b.y as i32 - self.a.y as i32) * (self.c.x as i32 - self.a.x as i32);

        let tria = self.a;
        let mut trib = self.b;
        let mut tric = self.c;

        if cross_product < 0 {
            // Swap b and c to make the order clockwise
            let temp = trib;
            trib = tric;
            tric = temp;
        }

        let min_x = tria.x.min(trib.x).min(tric.x);
        let max_x = tria.x.max(trib.x).max(tric.x);
        let min_y = tria.y.min(trib.y).min(tric.y);
        let max_y = tria.y.max(trib.y).max(tric.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x, y);

                let edge1 = (trib.x as i32 - tria.x as i32) * (p.y as i32 - tria.y as i32)
                    - (trib.y as i32 - tria.y as i32) * (p.x as i32 - tria.x as i32);
                let edge2 = (tric.x as i32 - trib.x as i32) * (p.y as i32 - trib.y as i32)
                    - (tric.y as i32 - trib.y as i32) * (p.x as i32 - trib.x as i32);
                let edge3 = (tria.x as i32 - tric.x as i32) * (p.y as i32 - tric.y as i32)
                    - (tria.y as i32 - tric.y as i32) * (p.x as i32 - tric.x as i32);

                if edge1 >= 0 && edge2 >= 0 && edge3 >= 0 {
                    target.put_pixel(Vec2::new(x, y), ColA::BLUE).unwrap();
                }
            }
        }

        Ok(())
    }
}
