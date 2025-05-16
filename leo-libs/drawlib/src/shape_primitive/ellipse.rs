use corelib::types::Float;
use mathlib::vectors::Vec2;
use renderlib::primitive::Primitive;

use crate::{drawable::Drawable, path::Path};

/// https://www.w3.org/TR/SVG2/shapes.html#EllipseElement
pub struct Ellipse(Path);

impl Ellipse {
    pub fn new(cx: Float, cy: Float, rx: Float, ry: Float) -> Self {
        let mut path = Path::new();

        // A move-to command to the point cx+rx,cy;
        path.move_to(Vec2::new(cx + rx, cy));

        // arc to cx,cy+ry;
        path.arc_to(Vec2::new(rx, ry), 0.0, 0.0, 1.0, Vec2::new(cx, cy + ry));

        // arc to cx-rx,cy;
        path.arc_to(Vec2::new(rx, ry), 0.0, 0.0, 1.0, Vec2::new(cx - rx, cy));

        // arc to cx,cy-ry;
        path.arc_to(Vec2::new(rx, ry), 0.0, 0.0, 1.0, Vec2::new(cx, cy - ry));

        // arc with a segment-completing close path operation.
        path.arc_to(Vec2::new(rx, ry), 0.0, 0.0, 1.0, Vec2::new(cx + rx, cy));
        path.close_path();

        Self(path)
    }
}

impl Drawable for Ellipse {
    fn to_primitives(self) -> Vec<Primitive> {
        self.0.to_primitives()
    }
}
