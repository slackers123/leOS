use std::io::Write;

use corelib::reader::{ByteReader, Readable, Reader};

use crate::Rgba;

#[derive(Debug)]
pub struct QoiHeader {
    pub width: u32,   // image width in pixels (BE)
    pub height: u32,  // image height in pixels (BE)
    pub channels: u8, // 3 = RGB, 4 = RGBA
    pub colorspace: u8, // 0 = sRGB with linear alpha
                      // 1 = all channels linear
}

impl Readable for QoiHeader {
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        let magic: [u8; 4] = reader.read();

        assert_eq!(&magic, b"qoif", "This is not a qoi file.");

        QoiHeader {
            width: reader.read(),
            height: reader.read(),
            channels: reader.read(),
            colorspace: reader.read(),
        }
    }
}

pub struct QoiWriter<'out, 'src, W: Write> {
    out: &'out mut W,
    image: &'src [Rgba],
    header: QoiHeader,
    pix_arr: [Rgba; 64],
    previous_pixel: Rgba,
    index: usize,
}

impl<'out, 'src, W: Write> QoiWriter<'out, 'src, W> {
    pub fn new(header: QoiHeader, image: &'src [Rgba], out: &'out mut W) -> Self {
        Self {
            out,
            image,
            header,
            pix_arr: [Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }; 64],
            previous_pixel: Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            index: 0,
        }
    }

    pub fn write(&mut self) {
        self.out.write_all(b"qoif").unwrap();
        self.out
            .write_all(&self.header.width.to_be_bytes())
            .unwrap();
        self.out
            .write_all(&self.header.height.to_be_bytes())
            .unwrap();
        self.out
            .write_all(&self.header.channels.to_be_bytes())
            .unwrap();
        self.out
            .write_all(&self.header.colorspace.to_be_bytes())
            .unwrap();

        if self.header.channels == 3 {
            while self.index < self.image.len() {
                self.write_next_chunk_rgb();
            }
        } else if self.header.channels == 4 {
            while self.index < self.image.len() {
                self.write_next_chunk_rgba();
            }
        } else {
            panic!("invalid channel amount");
        }

        self.out.write_all(&[0, 0, 0, 0, 0, 0, 0, 0x01]).unwrap();
    }

    pub fn write_next_chunk_rgb(&mut self) {
        let current_pix = self.image[self.index];
        if current_pix == self.previous_pixel {
            let mut cnt = 0;

            while self.index + cnt < self.image.len()
                && self.image[self.index + cnt] == self.previous_pixel
                && cnt < 62
            {
                cnt += 1;
            }

            self.index += cnt;
            self.out
                .write_all(&[(0b11 << 6) | (cnt - 1) as u8])
                .unwrap();

            return;
        }

        if self.color_is_registered(current_pix) {
            let index_position = (current_pix.r as usize * 3
                + current_pix.g as usize * 5
                + current_pix.b as usize * 7
                + current_pix.a as usize * 11)
                % 64;

            self.previous_pixel = current_pix;
            self.out
                .write_all(&[(0b00 << 6) | index_position as u8])
                .unwrap();
            self.index += 1;
            return;
        }

        if current_pix.a == self.previous_pixel.a {
            let r_diff = current_pix.r.wrapping_sub(self.previous_pixel.r) as i8;
            let g_diff = current_pix.g.wrapping_sub(self.previous_pixel.g) as i8;
            let b_diff = current_pix.b.wrapping_sub(self.previous_pixel.b) as i8;

            if (r_diff <= 1 && r_diff >= -2)
                && (g_diff <= 1 && g_diff >= -2)
                && (b_diff <= 1 && b_diff >= -2)
            {
                self.previous_pixel = current_pix;
                self.reg_pixel(self.previous_pixel);

                self.out
                    .write_all(&[(0b01 << 6)
                        | (((r_diff + 2) as u8) << 4)
                        | (((g_diff + 2) as u8) << 2)
                        | (((b_diff + 2) as u8) << 0)])
                    .unwrap();
                self.index += 1;
                return;
            }

            if g_diff <= 31 && g_diff >= -32 {
                let rg_diff = r_diff - g_diff;
                let bg_diff = b_diff - g_diff;

                if (rg_diff <= 7 && rg_diff >= -8) && (bg_diff <= 7 && bg_diff >= -8) {
                    self.previous_pixel = current_pix;
                    self.reg_pixel(self.previous_pixel);
                    self.out
                        .write_all(&[
                            (0b10 << 6) | (g_diff + 32) as u8,
                            ((rg_diff + 8) << 4) as u8 | ((bg_diff + 8) << 0) as u8,
                        ])
                        .unwrap();
                    self.index += 1;
                    return;
                }
            }
        }

        self.previous_pixel = current_pix;
        self.reg_pixel(self.previous_pixel);
        self.out
            .write_all(&[0b11111110, current_pix.r, current_pix.g, current_pix.b])
            .unwrap();

        self.index += 1;
    }

    pub fn write_next_chunk_rgba(&mut self) {
        let current_pix = self.image[self.index];
        if current_pix == self.previous_pixel {
            let mut cnt = 0;

            while self.index + cnt < self.image.len()
                && self.image[self.index + cnt] == self.previous_pixel
                && cnt < 62
            {
                cnt += 1;
            }

            self.index += cnt;
            self.out
                .write_all(&[(0b11 << 6) | (cnt - 1) as u8])
                .unwrap();

            return;
        }

        if self.color_is_registered(current_pix) {
            let index_position = (current_pix.r as usize * 3
                + current_pix.g as usize * 5
                + current_pix.b as usize * 7
                + current_pix.a as usize * 11)
                % 64;

            self.previous_pixel = current_pix;
            self.out
                .write_all(&[(0b00 << 6) | index_position as u8])
                .unwrap();
            self.index += 1;
            return;
        }

        if current_pix.a == self.previous_pixel.a {
            let r_diff = current_pix.r.wrapping_sub(self.previous_pixel.r) as i8;
            let g_diff = current_pix.g.wrapping_sub(self.previous_pixel.g) as i8;
            let b_diff = current_pix.b.wrapping_sub(self.previous_pixel.b) as i8;

            if (r_diff <= 1 && r_diff >= -2)
                && (g_diff <= 1 && g_diff >= -2)
                && (b_diff <= 1 && b_diff >= -2)
            {
                self.previous_pixel = current_pix;
                self.reg_pixel(self.previous_pixel);

                self.out
                    .write_all(&[(0b01 << 6)
                        | (((r_diff + 2) as u8) << 4)
                        | (((g_diff + 2) as u8) << 2)
                        | (((b_diff + 2) as u8) << 0)])
                    .unwrap();
                self.index += 1;
                return;
            }

            if g_diff <= 31 && g_diff >= -32 {
                let rg_diff = r_diff - g_diff;
                let bg_diff = b_diff - g_diff;

                if (rg_diff <= 7 && rg_diff >= -8) && (bg_diff <= 7 && bg_diff >= -8) {
                    self.previous_pixel = current_pix;
                    self.reg_pixel(self.previous_pixel);
                    self.out
                        .write_all(&[
                            (0b10 << 6) | (g_diff + 32) as u8,
                            ((rg_diff + 8) << 4) as u8 | ((bg_diff + 8) << 0) as u8,
                        ])
                        .unwrap();
                    self.index += 1;
                    return;
                }
            }
        }

        self.previous_pixel = current_pix;
        self.reg_pixel(self.previous_pixel);
        self.out
            .write_all(&[
                0b11111111,
                current_pix.r,
                current_pix.g,
                current_pix.b,
                current_pix.a,
            ])
            .unwrap();

        self.index += 1;
    }

    pub fn reg_pixel(&mut self, pix: Rgba) {
        let index_position =
            (pix.r as usize * 3 + pix.g as usize * 5 + pix.b as usize * 7 + pix.a as usize * 11)
                % 64;

        self.pix_arr[index_position] = pix;
    }

    #[inline]
    pub fn color_is_registered(&mut self, pix: Rgba) -> bool {
        let index_position =
            (pix.r as usize * 3 + pix.g as usize * 5 + pix.b as usize * 7 + pix.a as usize * 11)
                % 64;

        self.pix_arr[index_position] == pix
    }
}
