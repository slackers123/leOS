use corelib::types::Float;
use mathlib::vectors::Vec2;
use ttflib::Font;

use crate::path::Path;

pub fn get_char_path(c: char, font: &Font) -> Path {
    // TODO: improve this / make it more performant (lots of low hanging fruit)
    let glyf = font.get_glyph(c);
    let mut path = Path::new();
    let mut last_was_on_curve = false;
    let mut last_off_curve = Vec2::ZERO;
    for ((i, (x, y)), flags) in glyf
        .x_coordinates
        .iter()
        .zip(glyf.y_coordinates)
        .enumerate()
        .zip(glyf.flags)
    {
        let p = Vec2::new(*x as Float, 2000.0 - y as Float) / 4.0;

        if i == 0 || glyf.end_pts_of_contours.contains(&(i as u16 - 1)) {
            path.move_to(p);
        } else {
            if flags.on_curve_point && last_was_on_curve {
                path.line_to(p);
            } else if flags.on_curve_point {
                path.q_bezier_to(last_off_curve, p);
            } else {
                if !last_was_on_curve {
                    let last_target = (p - last_off_curve) / 2.0 + last_off_curve;
                    path.q_bezier_to(last_off_curve, last_target);
                }
                last_off_curve = p;
            }
        }

        if glyf.end_pts_of_contours.contains(&(i as u16)) {
            path.close_path();
        }

        last_was_on_curve = flags.on_curve_point;
    }
    path
}
