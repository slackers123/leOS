use corelib::types::Uint;
use drawlib::draw_target::DrawTarget;
use imglib::Rgba;
use mathlib::vectors::Vec2;

pub struct Qimg {
    data: Vec<Rgba>,
    width: u32,
    height: u32,
}

impl DrawTarget for Qimg {
    fn put_pixel(&mut self, pos: Vec2<Uint>, col: mathlib::color::ColA) {
        if pos.x < self.width && pos.y < self.height {}
    }
    fn dimensions(&self) -> (usize, usize) {
        (
            self.0.dimensions().0 as usize,
            self.0.dimensions().1 as usize,
        )
    }
}
