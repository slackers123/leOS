use crate::math::Splat;

pub struct BoxExtend<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Copy> Splat<T> for BoxExtend<T> {
    fn splat(v: T) -> Self {
        Self {
            left: v,
            right: v,
            top: v,
            bottom: v,
        }
    }
}
