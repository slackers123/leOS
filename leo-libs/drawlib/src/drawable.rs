use crate::{draw_target::DrawTarget, rendererror::RenderResult};

pub trait Drawable {
    fn draw(&self, target: &mut impl DrawTarget) -> RenderResult<()>;
}
