use corelib::types::Float;
use mathlib::{color::ColA, vectors::Vec2F};

pub struct Primitve {
    pub mesh: Mesh,
    pub material: Material,
}

pub struct Mesh {
    pub vertices: Vec<Vec2F>,
    pub indices: Vec<usize>,
}

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
