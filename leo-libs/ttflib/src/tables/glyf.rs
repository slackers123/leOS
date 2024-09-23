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

        let mut flags_arr = vec![];
        let mut i = 0;
        loop {
            if i >= num_points {
                break;
            }
            let flags = reader.read::<GlyphFlags>();
            //println!("{flags:b}");
            if flags.repeat_flag {
                let num = reader.read_byte();
                for _ in 0..num as u16 + 1 {
                    flags_arr.push(flags);
                }
            } else {
                flags_arr.push(flags);
            }

            i += 1;
        }

        let mut x_coordinates: Vec<i32> = Vec::with_capacity(flags_arr.len());
        for i in 0..flags_arr.len() {
            let last = x_coordinates.last().unwrap_or(&0);
            let flags = flags_arr[i];
            if flags.x_short_vector {
                let v = reader.read_byte();
                if flags.x_is_same_or_positive_x_short_vector {
                    x_coordinates.push(last + v as i32);
                } else {
                    x_coordinates.push(last - v as i32);
                }
            } else {
                if flags.x_is_same_or_positive_x_short_vector {
                    x_coordinates.push(*last);
                } else {
                    x_coordinates.push(*last + reader.read::<i16>() as i32);
                }
            }
        }

        let mut y_coordinates: Vec<i32> = Vec::with_capacity(flags_arr.len());
        for i in 0..flags_arr.len() {
            let last = y_coordinates.last().unwrap_or(&0);
            let flags = flags_arr[i];
            if flags.y_short_vector {
                let v = reader.read_byte();
                if flags.y_is_same_or_positive_y_short_vector {
                    y_coordinates.push(last + v as i32);
                } else {
                    y_coordinates.push(last - v as i32);
                }
            } else {
                if flags.y_is_same_or_positive_y_short_vector {
                    y_coordinates.push(*last);
                } else {
                    y_coordinates.push(last + reader.read::<i16>() as i32);
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
    pub end_pts_of_contours: Vec<u16>,
    pub instruction_length: u16,
    pub instructions: Vec<u8>,
    pub flags: Vec<GlyphFlags>,
    pub x_coordinates: Vec<i32>,
    pub y_coordinates: Vec<i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphFlags {
    pub on_curve_point: bool,
    pub x_short_vector: bool,
    pub y_short_vector: bool,
    pub repeat_flag: bool,
    pub x_is_same_or_positive_x_short_vector: bool,
    pub y_is_same_or_positive_y_short_vector: bool,
    pub overlap_simple: bool,
}

impl Readable for GlyphFlags {
    fn read(reader: &mut Reader<impl corelib::reader::ByteReader>) -> Self {
        let flags = reader.read_byte();
        const ON_CURVE_POINT: u8 = 0x01;
        const X_SHORT_VECTOR: u8 = 0x02;
        const Y_SHORT_VECTOR: u8 = 0x04;
        const REPEAT_FLAG: u8 = 0x08;
        const X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR: u8 = 0x10;
        const Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR: u8 = 0x20;
        const OVERLAP_SIMPLE: u8 = 0x40;

        Self {
            on_curve_point: flags & ON_CURVE_POINT != 0,
            x_short_vector: flags & X_SHORT_VECTOR != 0,
            y_short_vector: flags & Y_SHORT_VECTOR != 0,
            repeat_flag: flags & REPEAT_FLAG != 0,
            x_is_same_or_positive_x_short_vector: flags & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR != 0,
            y_is_same_or_positive_y_short_vector: flags & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR != 0,
            overlap_simple: flags & OVERLAP_SIMPLE != 0,
        }
    }
}
