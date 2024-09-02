use drawlib::{
    draw_target::DrawTarget, drawable::Drawable, line::Line2, path::Path, point::Point2,
    rect::Rect2, simpline::Simpline2,
};

#[allow(unused)]
pub enum Renderable {
    Line2(Line2),
    Rect2(Rect2),
    Point2(Point2),
    Simpline2(Simpline2),
    Path(Path),
}

impl Renderable {
    pub fn draw(&self, target: &mut impl DrawTarget) {
        match self {
            Self::Line2(v) => v.draw(target),
            Self::Rect2(v) => v.draw(target),
            Self::Point2(v) => v.draw(target),
            Self::Simpline2(v) => v.draw(target),
            Self::Path(v) => v.draw(target),
        }
    }
}
