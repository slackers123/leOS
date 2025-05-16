use renderlib::primitive::Primitive;

pub trait Drawable {
    fn to_primitives(self) -> Vec<Primitive>;
}
