use std::ops::Deref;

use crate::{number::Number, vectors::Vec2};

pub struct Triangle<T: Number> {
    pub a: Vec2<T>,
    pub b: Vec2<T>,
    pub c: Vec2<T>,
}

/// A Triangle that is always in clowise winding order
pub struct CWTriangle<T: Number>(Triangle<T>);

impl<T: Number> CWTriangle<T> {
    /// Directly creates a CWTriangle from a simple triangle. This is unsafe since the caller must guarantee
    /// the type's invariant of being in clockwise winding order.
    pub unsafe fn from_raw(tri: Triangle<T>) -> Self {
        Self(tri)
    }
}

impl<T: Number> From<Triangle<T>> for CWTriangle<T> {
    fn from(_value: Triangle<T>) -> Self {
        todo!("make sure the triangle is in clockwise order")
    }
}

impl<T: Number> Deref for CWTriangle<T> {
    type Target = Triangle<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
