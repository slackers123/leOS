use renderlib::primitive::Primitive;
use stroking::Path;

pub mod draw_target;
pub mod drawable;
pub mod path;
pub mod path_attr;
pub mod ptri;
pub mod shape_primitive;
pub mod stroking;
pub mod text;

pub fn tesselate(primitives: &[Path]) -> Vec<Primitive> {
    todo!()
}
