use crate::{
    border::Border,
    math::{ACol, Extent2, Vec2},
    padding::Padding,
    render::{Renderable, SimpleBuffer},
};

pub struct Rect {
    pub rel_pos: Vec2<u32>,
    pub content: Extent2<u32>,
    pub padding: Padding,
    pub border: Border,
    pub background_col: ACol<f32>,
}

impl Rect {
    #[inline]
    fn border_left_start(&self) -> u32 {
        0
    }

    #[inline]
    fn border_left_end(&self) -> u32 {
        self.border_left_start() + self.border.size.left
    }

    #[inline]
    fn padding_left_start(&self) -> u32 {
        self.border_left_end()
    }

    #[inline]
    fn padding_left_end(&self) -> u32 {
        self.padding_left_start() + self.padding.size.left
    }

    #[inline]
    fn content_horiz_start(&self) -> u32 {
        self.padding_left_end()
    }

    #[inline]
    fn content_horiz_end(&self) -> u32 {
        self.content_horiz_start() + self.content.width
    }

    #[inline]
    fn padding_right_start(&self) -> u32 {
        self.content_horiz_end()
    }

    #[inline]
    fn padding_right_end(&self) -> u32 {
        self.padding_right_start() + self.padding.size.right
    }

    #[inline]
    fn border_right_start(&self) -> u32 {
        self.padding_right_end()
    }

    #[inline]
    fn border_right_end(&self) -> u32 {
        self.border_right_start() + self.border.size.right
    }

    #[inline]
    fn border_top_start(&self) -> u32 {
        0
    }

    #[inline]
    fn border_top_end(&self) -> u32 {
        self.border_top_start() + self.border.size.top
    }

    #[inline]
    fn padding_top_start(&self) -> u32 {
        self.border_top_end()
    }

    #[inline]
    fn padding_top_end(&self) -> u32 {
        self.padding_top_start() + self.padding.size.top
    }

    #[inline]
    fn content_vert_start(&self) -> u32 {
        self.padding_top_end()
    }

    #[inline]
    fn content_vert_end(&self) -> u32 {
        self.content_vert_start() + self.content.height
    }

    #[inline]
    fn padding_bottom_start(&self) -> u32 {
        self.content_vert_end()
    }

    #[inline]
    fn padding_bottom_end(&self) -> u32 {
        self.padding_bottom_start() + self.padding.size.bottom
    }

    #[inline]
    fn border_bottom_start(&self) -> u32 {
        self.padding_bottom_end()
    }

    #[inline]
    fn border_bottom_end(&self) -> u32 {
        self.border_bottom_start() + self.border.size.bottom
    }
}

impl Renderable for Rect {
    fn render(&self, top_pos: Vec2<u32>, buffer: &mut SimpleBuffer) {
        // FIXME: this border implementation only works when all borders have equal thiccness
        let offset = self.rel_pos + top_pos;
        let rad = self.border.radius;

        let c = Vec2 {
            x: rad.top_left as f64,
            y: rad.top_left as f64,
        };

        // draw top left border upper
        for x in 0..rad.top_left {
            for y in 0..rad.top_left {
                let d = c.distance(Vec2 {
                    x: x as f64,
                    y: y as f64,
                });
                if d < rad.top_left as f64 && d > self.border_top_end() as f64 {
                    buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
                }
            }
        }

        // draw top border
        for x in rad.top_left..self.border_right_end() - rad.top_right {
            for y in 0..self.border_top_end() {
                buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
            }
        }

        let c = Vec2 {
            x: (self.border_right_end() - rad.top_right) as f64,
            y: rad.top_right as f64,
        };

        // draw top right corner
        for x in self.border_right_end() - rad.top_right..self.border_right_end() {
            for y in 0..rad.top_right {
                let d = c.distance(Vec2 {
                    x: x as f64,
                    y: y as f64,
                });
                if d > self.border_top_end() as f64 && d < rad.top_right as f64 {
                    buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
                }
            }
        }

        // draw right border
        for x in self.border_right_start()..self.border_right_end() {
            for y in rad.top_right..self.border_bottom_end() - rad.bottom_right {
                buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
            }
        }

        let c = Vec2 {
            x: (self.border_right_end() - rad.bottom_right) as f64,
            y: (self.border_bottom_end() - rad.bottom_right) as f64,
        };

        // draw bottom right corner
        for x in c.x as u32..self.border_right_end() {
            for y in c.y as u32..self.border_bottom_end() {
                let d = c.distance(Vec2 {
                    x: x as f64,
                    y: y as f64,
                });
                if d > self.border_top_end() as f64 && d < rad.top_right as f64 {
                    buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
                }
            }
        }

        // draw bottom border
        for x in rad.bottom_left..self.border_right_end() - rad.bottom_right {
            for y in self.border_bottom_start()..self.border_bottom_end() {
                buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
            }
        }

        let c = Vec2 {
            x: rad.bottom_left as f64,
            y: (self.border_bottom_end() - rad.bottom_left) as f64,
        };

        // draw bottom left corner
        for x in 0..rad.bottom_left {
            for y in c.y as u32..c.y as u32 + rad.bottom_left {
                let d = c.distance(Vec2 {
                    x: x as f64,
                    y: y as f64,
                });
                if d > self.border_top_end() as f64 && d < rad.top_right as f64 {
                    buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
                }
            }
        }

        // draw left border
        for x in 0..self.border_left_end() {
            for y in rad.top_left..self.border_bottom_end() - rad.bottom_left {
                buffer.set_pix(offset.x + x, offset.y + y, self.border.col);
            }
        }
    }
}
