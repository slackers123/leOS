use corelib::reader::{read_vec, Readable, Reader};

use super::{cmap::CMAPSubtable, loca::LocaTable};

pub fn get_glyf(
    glyf_global_offset: usize,
    src: &[u8],
    c: char,
    loca_table: &LocaTable,
    cmap_table: &CMAPSubtable,
) -> GlyphTable {
    let mut reader = Reader::new_big_endian(src, 0);
    let char_id = cmap_table.get_char_id(&mut reader, c);
    println!("{char_id}");
    let offset = loca_table.get_char_id_offset(char_id);
    println!("glyf_global_offset: {glyf_global_offset}");
    println!("offset: {offset}");
    let real_offset = glyf_global_offset + offset as usize;
    println!("read_offset: {real_offset}");
    reader.set_pos(real_offset);

    let header: GlyfHeader = reader.read();
    println!("{header:?}");

    if header.number_of_contours > 0 {
        let end_pts_of_contours: Vec<u16> =
            read_vec(&mut reader, header.number_of_contours as usize);
        let num_points = end_pts_of_contours.last().unwrap() + 1;
        let instruction_length: u16 = reader.read();
        let instructions: Vec<u8> = read_vec(&mut reader, instruction_length as usize);

        const ON_CURVE_POINT: u8 = 0x01;
        const X_SHORT_VECTOR: u8 = 0x02;
        const Y_SHORT_VECTOR: u8 = 0x04;
        const REPEAT_FLAG: u8 = 0x08;
        const X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR: u8 = 0x10;
        const Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR: u8 = 0x20;
        const OVERLAP_SIMPLE: u8 = 0x40;

        let mut flags_arr: Vec<u8> = vec![];
        let mut i = 0;
        loop {
            if i >= num_points {
                break;
            }
            let flags = reader.read_byte();
            //println!("{flags:b}");
            if (flags & REPEAT_FLAG) != 0 {
                let num = reader.read_byte();
                for _ in 0..num as u16 + 1 {
                    flags_arr.push(flags);
                }
            } else {
                flags_arr.push(flags);
            }

            i += 1;
        }

        let mut x_coordinates: Vec<i16> = Vec::with_capacity(flags_arr.len());
        for i in 0..flags_arr.len() {
            let flags = flags_arr[i];
            if flags & X_SHORT_VECTOR != 0 {
                let v = reader.read_byte();
                if flags & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR != 0 {
                    x_coordinates.push(v as i16);
                } else {
                    x_coordinates.push(-(v as i16));
                }
            } else {
                if flags & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR != 0 {
                    x_coordinates.push(*x_coordinates.last().unwrap());
                } else {
                    x_coordinates.push(reader.read());
                }
            }
        }

        let mut y_coordinates: Vec<i16> = Vec::with_capacity(flags_arr.len());
        for i in 0..flags_arr.len() {
            let flags = flags_arr[i];
            if flags & Y_SHORT_VECTOR != 0 {
                let v = reader.read_byte();
                if flags & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR != 0 {
                    y_coordinates.push(v as i16);
                } else {
                    y_coordinates.push(-(v as i16));
                }
            } else {
                if flags & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR != 0 {
                    y_coordinates.push(*x_coordinates.last().unwrap());
                } else {
                    y_coordinates.push(reader.read());
                }
            }
        }

        GlyphTable {
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags: flags_arr,
            x_coordinates,
            y_coordinates,
        }
    } else {
        todo!("multiple contours")
    }
}

#[derive(Debug)]
pub struct GlyfHeader {
    number_of_contours: i16,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
}

impl Readable for GlyfHeader {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            number_of_contours: reader.read(),
            x_min: reader.read(),
            y_min: reader.read(),
            x_max: reader.read(),
            y_max: reader.read(),
        }
    }
}

#[derive(Debug)]
pub struct GlyphTable {
    end_pts_of_contours: Vec<u16>,
    instruction_length: u16,
    instructions: Vec<u8>,
    flags: Vec<u8>,
    x_coordinates: Vec<i16>,
    y_coordinates: Vec<i16>,
}
