use corelib::types::Float;
use mathlib::vectors::Vec2;

use crate::{drawable::Drawable, path::Path};

/// https://www.w3.org/TR/SVG2/shapes.html#RectElement
pub struct Rect(Path);

impl Rect {
    /// https://www.w3.org/TR/SVG2/shapes.html#RectElement
    pub fn new(x: Float, y: Float, width: Float, height: Float, rx: Float, ry: Float) -> Self {
        let mut path = crate::path::Path::new();
        // perform an absolute moveto operation to location (x+rx,y);
        path.move_to(Vec2::new(x + rx, y));

        // perform an absolute horizontal lineto with parameter x+width-rx;
        path.hline_to(x + width - rx);

        // if both rx and ry are greater than zero
        if rx > 0.0 && ry > 0.0 {
            // perform an absolute elliptical arc operation
            // to coordinate (x+width,y+ry),
            // where rx and ry are used as the equivalent parameters to the elliptical arc command,
            // the x-axis-rotation and large-arc-flag are set to zero, the sweep-flag is set to one;

            path.arc_to(
                Vec2::new(rx, ry),
                0.0,
                0.0,
                1.0,
                Vec2::new(x + width, y + ry),
            );
        }

        // perform an absolute vertical lineto parameter y+height-ry;
        path.vline_to(y + height - ry);

        // if both rx and ry are greater than zero,
        if rx > 0.0 && ry > 0.0 {
            // perform an absolute elliptical arc operation
            // to coordinate (x+width-rx,y+height),
            // using the same parameters as previously;

            path.arc_to(
                Vec2::new(rx, ry),
                0.0,
                0.0,
                1.0,
                Vec2::new(x + width - rx, y + height),
            );
        }

        // perform an absolute horizontal lineto parameter x+rx;
        path.hline_to(x + rx);

        // if both rx and ry are greater than zero,
        if rx > 0.0 && ry > 0.0 {
            // perform an absolute elliptical arc operation
            // to coordinate (x,y+height-ry),
            // using the same parameters as previously;

            path.arc_to(
                Vec2::new(rx, ry),
                0.0,
                0.0,
                1.0,
                Vec2::new(x, y + height - ry),
            );
        }

        // perform an absolute vertical lineto parameter y+ry
        path.vline_to(y + ry);

        // if both rx and ry are greater than zero,
        if rx > 0.0 && ry > 0.0 {
            // perform an absolute elliptical arc operation
            // with a segment-completing close path operation,
            // using the same parameters as previously.

            path.arc_to(Vec2::new(rx, ry), 0.0, 0.0, 1.0, Vec2::new(x + rx, y));
        }

        path.close_path();

        Self(path)
    }
}

impl Drawable for Rect {
    fn draw(&self, target: &mut impl crate::draw_target::DrawTarget) {
        self.0.draw(target);
    }
}
