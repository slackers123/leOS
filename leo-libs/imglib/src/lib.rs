use std::{io::Write, marker::PhantomData};

pub mod qoi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const BLACK: Self = Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub type RgbaImage = Image<Rgba>;

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

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: Pix) {
        self.data[y * self.width + x] = pixel;
    }
}

impl Image<Rgba> {
    pub fn save(self, target: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create("test.qoi").unwrap();
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
