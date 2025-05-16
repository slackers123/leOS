use corelib::types::Float;
use mathlib::color::ColA;

/// The material of a Mesh or Point Strip
pub enum Material {
    SingleColor(ColA),
    SimpleGradient {
        color1: ColA,
        color2: ColA,
        direction: Float,
        size: Float,
    },
    Texture(Texture),
}

pub struct Texture;
