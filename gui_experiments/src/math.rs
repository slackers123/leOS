use std::ops::{Add, AddAssign, Mul, MulAssign};

use mathlib::vector::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Extent2<T: Copy> {
    pub pos: Vec2<T>,
    pub width: T,
    pub height: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: MulAssign + Copy> Mul<T> for Vec4<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
        return self;
    }
}

impl<T: AddAssign> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn add(mut self, rhs: Vec4<T>) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
        return self;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Col<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T: Copy> Splat<T> for Corners<T> {
    fn splat(v: T) -> Self {
        Corners {
            top_left: v,
            top_right: v,
            bottom_right: v,
            bottom_left: v,
        }
    }
}

pub trait Splat<T> {
    fn splat(v: T) -> Self;
}
