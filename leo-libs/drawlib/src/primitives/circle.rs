use corelib::types::Float;
use mathlib::vectors::Vec2;

use crate::{drawable::Drawable, path::Path};

/// https://www.w3.org/TR/SVG2/shapes.html#CircleElement
pub struct Circle(Path);

impl Circle {
    /// https://www.w3.org/TR/SVG2/shapes.html#CircleElement
    // FIXME: the document calls for the sweep flag to be zero but this does not make a lot
    // of sense
    pub fn new(cx: Float, cy: Float, r: Float) -> Self {
        let mut path = Path::new();

        // A move-to command to the point cx+r,cy;
        path.move_to(Vec2::new(cx + r, cy));

        // arc to cx,cy+r;
        path.arc_to(Vec2::splat(r), 0.0, 0.0, 1.0, Vec2::new(cx, cy + r));

        // arc to cx-r,cy;
        path.arc_to(Vec2::splat(r), 0.0, 0.0, 1.0, Vec2::new(cx - r, cy));

        // arc to cx,cy-r;
        path.arc_to(Vec2::splat(r), 0.0, 0.0, 1.0, Vec2::new(cx, cy - r));

        // arc with a segment-completing close path operation.
        path.arc_to(Vec2::splat(r), 0.0, 0.0, 1.0, Vec2::new(cx + r, cy));
        path.close_path();

        Self(path)
    }
}

impl Drawable for Circle {
    fn draw(
        &self,
        target: &mut impl crate::draw_target::DrawTarget,
    ) -> crate::rendererror::RenderResult<()> {
        self.0.draw(target)
    }
}
