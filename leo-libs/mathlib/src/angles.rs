use core::fmt;
use std::{f32::consts::PI, ops};

use corelib::types::Float;

pub const PI2: Float = PI * 2.;

pub trait Angle {
    fn sin(self) -> Float;
    fn cos(self) -> Float;
    fn tan(self) -> Float;
    fn asin(self) -> Float;
    fn acos(self) -> Float;
    fn atan(self) -> Float;
}

/// Struct that guarantees angles between 0..2PI
#[derive(Copy, Clone, PartialEq)]
pub struct Rad(Float);

impl Rad {
    pub fn new(rad: Float) -> Self {
        let rad = rad % PI2;
        if rad.is_sign_negative() {
            return Self(PI2 + rad);
        }
        return Self(rad);
    }

    pub fn from_deg(deg: Deg) -> Self {
        return Self(deg.as_float() / 180. * PI);
    }

    pub fn as_float(self) -> Float {
        self.0
    }
}

impl Angle for Rad {
    fn sin(self) -> Float {
        self.0.sin()
    }
    fn cos(self) -> Float {
        self.0.cos()
    }
    fn tan(self) -> Float {
        self.0.tan()
    }
    fn asin(self) -> Float {
        self.0.asin()
    }
    fn acos(self) -> Float {
        self.0.acos()
    }
    fn atan(self) -> Float {
        self.0.atan()
    }
}

/// Struct that guarantees angles between 0..360
#[derive(Copy, Clone, PartialEq)]
pub struct Deg(Float);

impl Deg {
    pub fn new(deg: Float) -> Self {
        let deg = deg % 360.;
        if deg.is_sign_negative() {
            return Self(360. + deg);
        }
        return Self(deg);
    }

    pub fn from_rad(rad: Rad) -> Self {
        Self((rad.as_float() / PI) * 180.)
    }

    pub fn as_float(self) -> Float {
        self.0
    }
}

impl Angle for Deg {
    fn sin(self) -> Float {
        Rad::from_deg(self).sin()
    }
    fn cos(self) -> Float {
        Rad::from_deg(self).cos()
    }
    fn tan(self) -> Float {
        Rad::from_deg(self).tan()
    }
    fn asin(self) -> Float {
        Rad::from_deg(self).asin()
    }
    fn acos(self) -> Float {
        Rad::from_deg(self).acos()
    }
    fn atan(self) -> Float {
        Rad::from_deg(self).atan()
    }
}

impl fmt::Debug for Rad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}rad", self.0)
    }
}

impl fmt::Display for Rad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}rad", self.0)
    }
}

impl fmt::Debug for Deg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}deg", self.0)
    }
}

impl fmt::Display for Deg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}deg", self.0)
    }
}

impl ops::Add<Rad> for Rad {
    type Output = Rad;
    fn add(self, rhs: Rad) -> Self::Output {
        let rad = self.0 + rhs.0;
        Self::new(rad)
    }
}

impl ops::Sub<Rad> for Rad {
    type Output = Rad;
    fn sub(self, rhs: Rad) -> Self::Output {
        let rad = self.0 - rhs.0;
        Self::new(rad)
    }
}

impl ops::Add<Deg> for Deg {
    type Output = Deg;
    fn add(self, rhs: Deg) -> Self::Output {
        let deg = self.0 + rhs.0;
        Self::new(deg)
    }
}

impl ops::Sub<Deg> for Deg {
    type Output = Deg;
    fn sub(self, rhs: Deg) -> Self::Output {
        let deg = self.0 - rhs.0;
        Self::new(deg)
    }
}
