use std::fmt::Debug;

use crate::{draw_target::DrawTarget, primitive::Primitve};

pub trait Drawable {
    fn to_primitives(self) -> Vec<Primitve>;
}
