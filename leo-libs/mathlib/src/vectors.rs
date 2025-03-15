use std::ops::{self};

use corelib::types::Float;

use crate::{matrix::Mat, number::Number};

#[derive(Copy, PartialEq, Debug, Clone)]
pub struct Vec2<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: Number> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn to_mat(self) -> Mat<2, 1, T> {
        Mat::new([[self.x], [self.y]])
    }

    pub fn from_mat(mat: Mat<2, 1, T>) -> Self {
        Self::new(mat[0][0], mat[1][0])
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

    pub fn dot(&self, rhs: &Self) -> Float {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn angle_to(&self, rhs: &Self) -> Float {
        let lower = (self.dot(rhs) / (self.length() * rhs.length())).clamp(-1.0, 1.0);

        lower.acos() * (self.x * rhs.y - self.y * rhs.x).signum()
    }
}

impl<T: Number> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(mut self, rhs: Vec2<T>) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl<T: Number> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(mut self, rhs: Vec2<T>) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl<T: Number> ops::Mul<T> for Vec2<T> {
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

impl<T: Number> ops::Div<T> for Vec2<T> {
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

impl<T: Number> ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = *self + rhs;
    }
}

impl<T: Number> ops::SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        *self = *self - rhs;
    }
}

impl<T: Number> ops::Mul for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(mut self, rhs: Vec2<T>) -> Vec2<T> {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self
    }
}

impl<T: Number + ops::Neg<Output = T>> ops::Neg for Vec2<T> {
    type Output = Self;
    fn neg(mut self) -> Vec2<T> {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}
