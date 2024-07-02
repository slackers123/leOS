use drawlib::{
    bezier::Bezier, draw_target::DrawTarget, drawable::Drawable, line::Line2, rect::Rect2,
    tri::Tri2,
};
use mathlib::{
    color::ColA,
    types::{Float, Uint},
    vector::Vec2,
};

use crate::{
    border::Border,
    math::{Corners, Extent2, Splat},
    padding::Padding,
    rect::Rect,
};

pub struct Renderer {
    renderables: Vec<Box<dyn Renderable>>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            renderables: vec![
                Box::new(Rect {
                    rel_pos: Vec2 { x: 0, y: 0 },
                    content: Extent2 {
                        pos: Vec2::<Uint>::ZERO,
                        width: 100,
                        height: 100,
                    },
                    padding: Padding {
                        size: Splat::splat(0),
                    },
                    background_col: ColA {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 0.5,
                    },
                    border: Border {
                        radius: Corners::splat(20),
                        size: Splat::splat(10),
                        col: ColA {
                            r: 0.0,
                            g: 0.0,
                            b: 1.0,
                            a: 1.0,
                        },
                    },
                }),
                Box::new(Rect {
                    rel_pos: Vec2 { x: 50, y: 50 },
                    content: Extent2 {
                        pos: Vec2::<Uint>::ZERO,
                        width: 100,
                        height: 100,
                    },
                    padding: Padding {
                        size: Splat::splat(0),
                    },
                    background_col: ColA {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    },
                    border: Border {
                        radius: Splat::splat(0),
                        size: Splat::splat(0),
                        col: ColA {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        },
                    },
                }),
            ],
        }
    }
}

impl Renderer {
    pub fn render(&mut self, buffer: &mut [u32], width: u32, _height: u32) {
        let mut simple_buffer = SimpleBuffer::new(buffer, width);
        self.draw_to_buffer(&mut simple_buffer);
    }

    fn draw_to_buffer(&self, buffer: &mut SimpleBuffer) {
        let line = Line2::new(
            Vec2::new(50.0, 50.0),
            Vec2::new(500.0, 500.0),
            100.0,
            ColA::WHITE,
        );

        // let tri = Tri2::new(
        //     Vec2::new(20.0, 100.0),
        //     Vec2::new(50.0, 20.0),
        //     Vec2::new(100.0, 100.0),
        //     ColA::WHITE,
        // );
        line.draw(buffer);
    }
}

pub struct SimpleBuffer<'a> {
    buffer: &'a mut [u32],
    width: u32,
}

impl<'a> SimpleBuffer<'a> {
    pub fn new(buffer: &'a mut [u32], width: u32) -> SimpleBuffer<'a> {
        Self { buffer, width }
    }

    pub fn set_pix(&mut self, x: u32, y: u32, col: ColA) {
        let index = y * self.width + x;
        let old = self.buffer[index as usize];
        let old_r = ((old >> 16) & 0xFF) as f32 / 255.0;
        let old_g = ((old >> 8) & 0xFF) as f32 / 255.0;
        let old_b = ((old >> 0) & 0xFF) as f32 / 255.0;
        let old = ColA {
            r: old_r,
            g: old_g,
            b: old_b,
            a: 0.0,
        };

        let col = col * col.a + old * (1.0 - col.a);
        let r = (col.r * 255.0) as u32;
        let g = (col.g * 255.0) as u32;
        let b = (col.b * 255.0) as u32;
        self.buffer[index as usize] = b | (g << 8) | (r << 16);
    }
}

impl<'a> DrawTarget for SimpleBuffer<'a> {
    fn put_px(&mut self, pix: Vec2<Uint>, col: ColA) {
        self.set_pix(pix.x, pix.y, col)
    }
}

pub trait Renderable {
    fn render(&self, top_pos: Vec2<u32>, buffer: &mut SimpleBuffer);
}
