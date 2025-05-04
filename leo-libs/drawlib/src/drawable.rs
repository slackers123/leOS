use crate::draw_target::DrawTarget;

pub trait Drawable {
    fn draw(&self, target: &mut impl DrawTarget);
}
