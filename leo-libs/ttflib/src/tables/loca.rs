use corelib::reader::{read_vec, ByteReader, Reader};

pub fn get_loca_table(
    reader: &mut Reader<impl ByteReader>,
    num_glyphs: usize,
    index_to_loc_format: i16,
) -> LocaTable {
    match index_to_loc_format {
        0 => LocaTable {
            values: read_vec::<u16>(reader, num_glyphs + 1)
                .into_iter()
                .map(|v| v as u32 * 2)
                .collect(),
        },
        1 => LocaTable {
            values: read_vec(reader, num_glyphs + 1),
        },
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct LocaTable {
    values: Vec<u32>,
}

impl LocaTable {
    pub fn get_char_id_offset(&self, char_id: usize) -> u32 {
        self.values[char_id]
    }
}
