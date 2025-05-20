use drawable::Drawable;
use path::Path;
use renderlib::primitive::Primitive;

pub mod draw_target;
pub mod drawable;
pub mod path;
pub mod path_attr;
pub mod ptri;
pub mod shape_primitive;
pub mod stroking;
pub mod text;

pub fn tesselate(primitives: &[Path]) -> Vec<Primitive> {
    primitives
        .into_iter()
        .flat_map(|p| p.to_stroke_path().to_primitives())
        .collect()
}
