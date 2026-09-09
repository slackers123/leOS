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

impl Material {
    pub fn get_color(&self) -> ColA {
        match self {
            Self::SingleColor(col) => *col,
            Self::SimpleGradient {
                color1,
                color2,
                direction,
                size,
            } => *color1,
            Self::Texture(t) => ColA::GREEN,
        }
    }
}

#[derive(Debug)]
pub struct Texture;
