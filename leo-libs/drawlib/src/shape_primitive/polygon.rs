use corelib::types::Float;
use mathlib::vectors::Vec2;

use crate::{drawable::Drawable, path::Path};

pub struct Polygon(Path);

impl Polygon {
    pub fn new(points: Vec<Float>) -> Self {
        let mut path = Path::new();

        if points.len() < 2 {
            return Self(path);
        }

        // perform an absolute moveto operation to the first coordinate pair in the list of points
        path.move_to(Vec2::new(points[0], points[1]));

        // for each subsequent coordinate pair, perform an absolute lineto operation to that coordinate pair.
        for pair in points.chunks(2).skip(1) {
            path.line_to(Vec2::new(pair[0], pair[1]));
        }

        // perform a closepath command
        path.close_path();

        Self(path)
    }
}

impl Drawable for Polygon {
    fn to_primitives(self) -> Vec<crate::primitive::Primitve> {
        self.0.to_primitives()
    }
}
