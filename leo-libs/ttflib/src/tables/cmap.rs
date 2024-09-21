use corelib::reader::{read_vec, Readable};

use crate::tabledir::TableRecord;

pub fn get_cmap(
    reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>,
    cmap_table_record: &TableRecord,
) -> CMAPSubtable {
    let offset = cmap_table_record.offset as usize;
    reader.set_pos(offset);
    let cmap = reader.read::<CmapHeader>();
    println!("{cmap:?}");

    reader.set_pos(
        offset
            + cmap
                .encoding_records
                .iter()
                .find(|er| er.platform_id == 0)
                .unwrap()
                .subtable_offset as usize,
    );
    let subtable = reader.read::<CMAPSubtable>();
    subtable
}

#[derive(Debug, Clone)]
struct CmapHeader {
    pub version: u16,
    pub num_tables: u16,
    pub encoding_records: Vec<EncodingRecord>,
}

impl Readable for CmapHeader {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        let version: u16 = reader.read();
        let num_tables: u16 = reader.read();
        let encoding_records: Vec<EncodingRecord> = read_vec(reader, num_tables as usize);
        Self {
            version,
            num_tables,
            encoding_records,
        }
    }
}

#[derive(Debug, Clone)]
struct EncodingRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub subtable_offset: u32,
}

impl Readable for EncodingRecord {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            platform_id: reader.read(),
            encoding_id: reader.read(),
            subtable_offset: reader.read(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CMAPSubtable {
    Format0(),
    Format2(),
    Format4(CMAPSubtableFormat4),
    Format6(),
    Format8(),
    Format10(),
    Format12(CMAPSubtableFormat12),
    Format13(),
    Format14(),
}

impl CMAPSubtable {
    pub fn get_char_id(
        &self,
        reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>,
        c: char,
    ) -> usize {
        match self {
            Self::Format4(sub) => sub.get_char_id(reader, c),
            _ => unimplemented!("subtable formats other than 4"),
        }
    }
}

impl Readable for CMAPSubtable {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        let format = reader.read::<u16>();
        match format {
            0 => Self::Format0(),
            2 => Self::Format2(),
            6 => Self::Format6(),
            8 => Self::Format8(),
            10 => Self::Format10(),
            13 => Self::Format13(),
            14 => Self::Format14(),
            4 => Self::Format4(reader.read::<CMAPSubtableFormat4>()),
            12 => Self::Format12(reader.read::<CMAPSubtableFormat12>()),
            _ => unimplemented!("format {format}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CMAPSubtableFormat4 {
    pub length: u16,
    pub language: u16,
    pub seg_count_x2: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub end_code: Vec<u16>,
    pub reserved_pad: u16,
    pub start_code: Vec<u16>,
    pub id_delta: Vec<i16>,
    pub id_range_offsets_start: usize,
    pub id_range_offset: Vec<u16>,
}

impl CMAPSubtableFormat4 {
    pub fn get_char_id(
        &self,
        reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>,
        c: char,
    ) -> usize {
        // Algorithm without pointer magic from: https://tchayen.github.io/posts/ttf-file-parsing
        let c_code = c as u16;

        let mut seg_idx = 0;
        for (id, e_code) in self.end_code.iter().enumerate() {
            if *e_code >= c_code && self.start_code[id] <= c_code {
                seg_idx = id;
                break;
            }
        }

        println!("seg_idx: {seg_idx}");

        if self.id_range_offset[seg_idx] == 0 {
            return ((c_code as i32 + self.id_delta[seg_idx] as i32) & 0xFFFF) as usize;
        }

        let start_code_offset = (c_code - self.start_code[seg_idx]) as usize * 2;
        println!("start_code_offset: {start_code_offset}");
        let current_range_offset = seg_idx * 2; // 2 because the numbers are 2 byte big.
        println!("current_range_offset: {current_range_offset}");

        let glyph_index_offset = self.id_range_offsets_start
            + current_range_offset
            + self.id_range_offset[seg_idx] as usize
            + (c_code as usize - self.start_code[seg_idx] as usize) * 2;

        reader.set_pos(glyph_index_offset);
        let index = reader.read::<u16>();
        (index as i32 + self.id_delta[seg_idx] as i32) as usize
    }
}

impl Readable for CMAPSubtableFormat4 {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        let length: u16 = reader.read();
        let language: u16 = reader.read();
        let seg_count_x2: u16 = reader.read();
        let seg_count = seg_count_x2 / 2;
        let search_range: u16 = reader.read();
        let entry_selector: u16 = reader.read();
        let range_shift: u16 = reader.read();
        let end_code: Vec<u16> = read_vec(reader, seg_count as usize);
        let reserved_pad: u16 = reader.read();
        let start_code: Vec<u16> = read_vec(reader, seg_count as usize);
        let id_delta: Vec<i16> = read_vec(reader, seg_count as usize);
        let id_range_offsets_start = reader.get_pos();
        let id_range_offset: Vec<u16> = read_vec(reader, seg_count as usize);

        Self {
            length,
            language,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            end_code,
            reserved_pad,
            start_code,
            id_delta,
            id_range_offsets_start,
            id_range_offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CMAPSubtableFormat12 {
    pub reserved: u16,
    pub length: u32,
    pub language: u32,
    pub num_groups: u32,
    pub groups: Vec<SequentialMapGroup>,
}

impl Readable for CMAPSubtableFormat12 {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        let reserved: u16 = reader.read();
        let length: u32 = reader.read();
        let language: u32 = reader.read();
        let num_groups: u32 = reader.read();
        let groups: Vec<SequentialMapGroup> = read_vec(reader, num_groups as usize);

        Self {
            reserved,
            length,
            language,
            num_groups,
            groups,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SequentialMapGroup {
    pub start_char_code: u32,
    pub end_char_code: u32,
    pub start_glyph_id: u32,
}

impl Readable for SequentialMapGroup {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            start_char_code: reader.read(),
            end_char_code: reader.read(),
            start_glyph_id: reader.read(),
        }
    }
}
