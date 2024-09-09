use corelib::reader::{read_vec, ByteReader, Readable, Reader};

use crate::util::FourByteTag;

#[derive(Debug, Clone)]
pub struct TableDirectory {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub table_records: Vec<TableRecord>,
}

impl Readable for TableDirectory {
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        let sfnt_version: u32 = reader.read();
        let num_tables: u16 = reader.read();
        let search_range: u16 = reader.read();
        let entry_selector: u16 = reader.read();
        let range_shift: u16 = reader.read();
        let table_records: Vec<TableRecord> = read_vec(reader, num_tables as usize);
        Self {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableRecord {
    pub table_tag: FourByteTag,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

impl Readable for TableRecord {
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        Self {
            table_tag: reader.read(),
            checksum: reader.read(),
            offset: reader.read(),
            length: reader.read(),
        }
    }
}
