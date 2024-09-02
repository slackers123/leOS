use drawlib::draw_target::DrawTarget;
use image::{ImageBuffer, Rgba};

pub struct Qimg(pub ImageBuffer<Rgba<u8>, Vec<u8>>);

impl DrawTarget for Qimg {
    fn put_px(
        &mut self,
        pix: mathlib::vector::Vec2<mathlib::types::Uint>,
        col: mathlib::color::ColA,
    ) {
        let dims = self.0.dimensions();
        if pix.x < dims.0 && pix.y < dims.1 {
            self.0.put_pixel(
                pix.x,
                pix.y,
                Rgba([
                    (col.r * 255.0) as u8,
                    (col.g * 255.0) as u8,
                    (col.b * 255.0) as u8,
                    (col.a * 255.0) as u8,
                ]),
            );
        }
    }
}
