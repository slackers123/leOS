use dpilib::PPos;
use mathlib::color::ColA;

pub trait DrawTarget {
    /// place a pixel at the specified position.
    /// If the position is outside of the range of the DrawTarget
    /// it may choose to either panic or just ignore it.
    fn put_pixel(&mut self, pos: PPos, col: ColA);
    fn dimensions(&self) -> (usize, usize);
}
