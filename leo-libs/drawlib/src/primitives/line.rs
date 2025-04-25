use corelib::types::Float;
use mathlib::vectors::Vec2;

use crate::{drawable::Drawable, path::Path};

/// https://www.w3.org/TR/SVG2/shapes.html#LineElement
pub struct Line(Path);

impl Line {
    /// https://www.w3.org/TR/SVG2/shapes.html#LineElement
    pub fn new(x1: Float, y1: Float, x2: Float, y2: Float) -> Self {
        let mut path = Path::new();

        // perform an absolute moveto operation to absolute location (x1,y1)
        path.move_to(Vec2::new(x1, y1));
        // perform an absolute lineto operation to absolute location (x2,y2)
        path.line_to(Vec2::new(x2, y2));

        Self(path)
    }
}

impl Drawable for Line {
    fn draw(
        &self,
        target: &mut impl crate::draw_target::DrawTarget,
    ) -> crate::rendererror::RenderResult<()> {
        self.0.draw(target)
    }
}
