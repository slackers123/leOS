use corelib::types::Uint;
use mathlib::vectors::Vec2;

use crate::drawable::Drawable;

/// A pixel triangle in screen space.
///
/// It is used internally to render almost everything
pub struct PTri {
    pub a: Vec2<Uint>,
    pub b: Vec2<Uint>,
    pub c: Vec2<Uint>,
}

impl PTri {
    pub fn new(a: Vec2<Uint>, b: Vec2<Uint>, c: Vec2<Uint>) -> Self {
        Self { a, b, c }
    }
}

impl Drawable for PTri {
    fn draw(
        &self,
        _target: &mut impl crate::draw_target::DrawTarget,
    ) -> crate::rendererror::RenderResult<()> {
        todo!("draw")
    }
}
