use std::ops::Mul;

use corelib::types::Float;

#[derive(Debug, Default, Clone, Copy)]
pub struct ColA {
    pub r: Float,
    pub g: Float,
    pub b: Float,
    pub a: Float,
}

impl ColA {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const PINK: Self = Self {
        r: 1.0,
        g: 0.7529411765,
        b: 0.7960784314,
        a: 1.0,
    };
    pub const LIGHT_BLUE: Self = Self {
        r: 0.678,
        g: 0.847,
        b: 0.902,
        a: 1.0,
    };
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub fn to_rgba_arr(self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}
