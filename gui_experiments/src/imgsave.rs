use corelib::types::Uint;
use drawlib::draw_target::DrawTarget;
use image::{ImageBuffer, Rgba};
use mathlib::vectors::Vec2;

pub struct Qimg(pub ImageBuffer<Rgba<u8>, Vec<u8>>);

impl DrawTarget for Qimg {
    fn put_pixel(
        &mut self,
        pos: Vec2<Uint>,
        col: mathlib::color::ColA,
    ) -> drawlib::rendererror::RenderResult<()> {
        self.0.put_pixel(pos.x, pos.y, Rgba(col.to_rgba_arr()));
        Ok(())
    }
}
