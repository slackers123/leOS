use corelib::reader::{BigEndianReader, Reader};

use crate::Rgba;

use super::writer::QoiHeader;

pub struct QoiReader<'data> {
    reader: &'data mut Reader<'data, BigEndianReader>,
    header: QoiHeader,
    pix_arr: [Rgba; 64],
    pub result: Vec<Rgba>,
    previous_pixel: Rgba,
}

impl<'data> QoiReader<'data> {
    pub fn new(reader: &'data mut Reader<'data, BigEndianReader>) -> Self {
        let header: QoiHeader = reader.read();
        Self {
            reader,
            result: Vec::with_capacity((header.height * header.width) as usize),
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
        }
    }

    pub fn read_entire_image(mut self) -> (QoiHeader, Vec<Rgba>) {
        let pic_size = self.header.width * self.header.height;

        while self.result.len() < pic_size as usize {
            self.read_chunk();
        }

        (self.header, self.result)
    }

    fn reg_new_pix(&mut self, pix: Rgba) {
        let index_position =
            (pix.r as usize * 3 + pix.g as usize * 5 + pix.b as usize * 7 + pix.a as usize * 11)
                % 64;

        self.pix_arr[index_position] = pix;
    }

    pub fn read_chunk(&mut self) {
        let first_byte = self.reader.read_byte();

        match first_byte {
            0b11111110 => {
                let red = self.reader.read_byte();
                let green = self.reader.read_byte();
                let blue = self.reader.read_byte();

                self.previous_pixel = Rgba {
                    r: red,
                    g: green,
                    b: blue,
                    a: self.previous_pixel.a,
                };

                self.reg_new_pix(self.previous_pixel);
                self.result.push(self.previous_pixel);
            }
            0b11111111 => {
                let red = self.reader.read_byte();
                let green = self.reader.read_byte();
                let blue = self.reader.read_byte();
                let alpha = self.reader.read_byte();

                self.previous_pixel = Rgba {
                    r: red,
                    g: green,
                    b: blue,
                    a: alpha,
                };
                self.reg_new_pix(self.previous_pixel);
                self.result.push(self.previous_pixel);
            }
            _ => match first_byte >> 6 {
                0b00 => {
                    let index = first_byte & 0b111111;

                    self.previous_pixel = self.pix_arr[index as usize];
                    self.reg_new_pix(self.previous_pixel);
                    self.result.push(self.previous_pixel);
                }
                0b01 => {
                    let new_r = (first_byte >> 4) & 0b11;
                    let new_g = (first_byte >> 2) & 0b11;
                    let new_b = (first_byte >> 0) & 0b11;

                    self.previous_pixel.r =
                        (self.previous_pixel.r.wrapping_add(new_r)).wrapping_sub(2);
                    self.previous_pixel.g =
                        (self.previous_pixel.g.wrapping_add(new_g)).wrapping_sub(2);
                    self.previous_pixel.b =
                        (self.previous_pixel.b.wrapping_add(new_b)).wrapping_sub(2);

                    self.reg_new_pix(self.previous_pixel);
                    self.result.push(self.previous_pixel);
                }
                0b10 => {
                    let next_byte = self.reader.read_byte();
                    let diff_green = (first_byte & 0b111111).wrapping_sub(32);

                    let dr_dg = (next_byte >> 4) & 0b1111;
                    let db_dg = (next_byte >> 0) & 0b1111;

                    let diff_red = diff_green.wrapping_add(dr_dg).wrapping_sub(8);
                    let diff_blue = diff_green.wrapping_add(db_dg).wrapping_sub(8);

                    self.previous_pixel.r = self.previous_pixel.r.wrapping_add(diff_red);
                    self.previous_pixel.g = self.previous_pixel.g.wrapping_add(diff_green);
                    self.previous_pixel.b = self.previous_pixel.b.wrapping_add(diff_blue);

                    self.reg_new_pix(self.previous_pixel);
                    self.result.push(self.previous_pixel);
                }
                0b11 => {
                    let run_length = first_byte & 0b111111;
                    assert!(run_length != 0b111111 && run_length != 0b111110);

                    // This handles the edge case of the first chunk being a run
                    if self.reader.get_pos() == 15 {
                        self.reg_new_pix(self.previous_pixel);
                    }

                    for _ in 0..(run_length + 1) {
                        self.result.push(self.previous_pixel);
                    }
                }
                _ => panic!("invalid qoi file"),
            },
        }
    }
}
