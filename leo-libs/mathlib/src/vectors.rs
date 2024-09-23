use std::ops::{self, AddAssign, DivAssign, MulAssign, SubAssign};

use corelib::types::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2<Float> {
    pub const INFINITY: Self = Vec2 {
        x: Float::INFINITY,
        y: Float::INFINITY,
    };
    pub const NEG_INFINITY: Self = Vec2 {
        x: Float::NEG_INFINITY,
        y: Float::NEG_INFINITY,
    };

    pub const ZERO: Self = Vec2 { x: 0.0, y: 0.0 };

    pub fn length(&self) -> Float {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        self.x /= l;
        self.y /= l;
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
}

impl<T: SubAssign> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(mut self, rhs: Vec2<T>) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl<T: AddAssign> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(mut self, rhs: Vec2<T>) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl<T: MulAssign + Copy> ops::Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(mut self, rhs: T) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl ops::Mul<Vec2<Float>> for Float {
    type Output = Vec2<Float>;
    fn mul(self, rhs: Vec2<Float>) -> Self::Output {
        rhs * self
    }
}

impl<T: DivAssign + Copy> ops::Div<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn div(mut self, rhs: T) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self
    }
}

impl ops::Div<Vec2<Float>> for Float {
    type Output = Vec2<Float>;
    fn div(self, rhs: Vec2<Float>) -> Self::Output {
        rhs / self
    }
}

impl<T: AddAssign + Copy> ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = *self + rhs;
    }
}
