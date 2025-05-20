use mathlib::vectors::Vec2F;

use crate::material::Material;

#[derive(Debug)]
pub struct Primitive {
    pub mesh: Mesh,
    pub material: Material,
}

#[derive(Debug)]
pub struct Mesh {
    pub ty: MeshType,
    pub vertices: Vec<Vec2F>,
    pub indices: Vec<usize>,
}

#[derive(Debug)]
pub enum MeshType {
    /// Simple triangles assembled using the vertices and indices
    Triangle,
    /// A triangle strip: https://en.wikipedia.org/wiki/Triangle_strip
    TriangleStrip,
    /// A list of vertices wihch are filled in as a shape using either
    /// a non-zero or even odd fill rule.
    FillShape { fill_rule: FillRule },
}

#[derive(Debug)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}
