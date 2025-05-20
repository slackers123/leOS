use corelib::types::Float;
use mathlib::color::ColA;

/// The material of a Mesh or Point Strip
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Texture;
