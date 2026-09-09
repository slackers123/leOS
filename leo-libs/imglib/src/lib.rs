use std::{io::Write, marker::PhantomData};

use mathlib::color::ColA;

pub mod qoi;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RgbaF32 {
    pub const BLACK: Self = Self {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 1.,
    };

    pub const TRANSPARENT: Self = Self {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };

    pub fn from_cola(col: ColA) -> Self {
        Self {
            r: col.r,
            g: col.g,
            b: col.b,
            a: col.a,
        }
    }

    pub fn to_u8(self) -> RgbaU8 {
        RgbaU8 {
            r: (self.r * 255.) as u8,
            g: (self.g * 255.) as u8,
            b: (self.b * 255.) as u8,
            a: (self.a * 255.) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbaU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaU8 {
    pub const BLACK: Self = RgbaU8 {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn from_cola(col: ColA) -> Self {
        Self {
            r: (col.r * 255.) as u8,
            g: (col.g * 255.) as u8,
            b: (col.b * 255.) as u8,
            a: (col.a * 255.) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub type RgbaU8Image = Image<RgbaU8>;

pub type RgbaF32Image = Image<RgbaF32>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image<Pix, Cont = Vec<Pix>> {
    pub data: Cont,
    pub width: usize,
    pub height: usize,
    _phant: PhantomData<Pix>,
}

impl<Pix: Copy> Image<Pix> {
    pub fn new(width: usize, height: usize, fill: Pix) -> Self {
        Self {
            data: vec![fill; width * height],
            width,
            height,
            _phant: PhantomData,
        }
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Pix {
        self.data[y * self.width + x]
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut Pix {
        &mut self.data[y * self.width + x]
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: Pix) {
        self.data[y * self.width + x] = pixel;
    }
}

impl Image<RgbaU8> {
    pub fn save(self, target: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(target).unwrap();
        let mut writer = qoi::writer::QoiWriter::new(
            qoi::writer::QoiHeader {
                width: self.width as u32,
                height: self.height as u32,
                channels: 4,
                colorspace: 1,
            },
            &self.data,
            &mut file,
        );
        writer.write();
        file.flush()?;

        Ok(())
    }
}

impl Image<RgbaF32> {
    pub fn to_u8(self) -> Image<RgbaU8> {
        Image {
            data: self.data.into_iter().map(|pix| pix.to_u8()).collect(),
            width: self.width,
            height: self.height,
            _phant: PhantomData,
        }
    }
}
