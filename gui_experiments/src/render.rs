use drawlib::draw_target::DrawTarget;
use mathlib::{color::ColA, types::Uint, vector::Vec2};

use crate::renderable::Renderable;

pub struct Renderer {
    pub renderables: Vec<Renderable>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            renderables: vec![],
        }
    }
}

impl Renderer {
    pub fn add_renderable(&mut self, renderable: Renderable) {
        self.renderables.push(renderable);
    }
    pub fn render(&self, buffer: &mut [u32], width: u32, height: u32) {
        let mut simple_buffer = SimpleBuffer::new(buffer, width, height);
        self.draw_to_buffer(&mut simple_buffer);
    }

    fn draw_to_buffer(&self, buffer: &mut SimpleBuffer) {
        for renderable in &self.renderables {
            renderable.draw(buffer);
        }
    }
}

pub struct SimpleBuffer<'a> {
    buffer: &'a mut [Uint],
    width: Uint,
    height: Uint,
}

impl<'a> SimpleBuffer<'a> {
    pub fn new(buffer: &'a mut [Uint], width: Uint, height: Uint) -> SimpleBuffer<'a> {
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn set_pix(&mut self, x: u32, y: u32, col: ColA) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        let old = self.buffer[index as usize];
        let old_r = ((old >> 16) & 0xFF) as f32 / 255.0;
        let old_g = ((old >> 8) & 0xFF) as f32 / 255.0;
        #[allow(clippy::identity_op)]
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
