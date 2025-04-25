use corelib::types::Uint;
use mathlib::{color::ColA, vectors::Vec2};

use crate::rendererror::RenderResult;

pub trait DrawTarget {
    fn put_pixel(&mut self, pos: Vec2<Uint>, col: ColA) -> RenderResult<()>;
    fn dimensions(&self) -> (usize, usize);
}
