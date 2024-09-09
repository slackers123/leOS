use corelib::reader::Readable;

pub struct HeadHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub font_revision: u32,
    pub checksum_adjustment: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: i64,
    pub modified: i64,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}

impl Readable for HeadHeader {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            major_version: reader.read(),
            minor_version: reader.read(),
            font_revision: reader.read(),
            checksum_adjustment: reader.read(),
            magic_number: reader.read(),
            flags: reader.read(),
            units_per_em: reader.read(),
            created: reader.read(),
            modified: reader.read(),
            x_min: reader.read(),
            y_min: reader.read(),
            x_max: reader.read(),
            y_max: reader.read(),
            mac_style: reader.read(),
            lowest_rec_ppem: reader.read(),
            font_direction_hint: reader.read(),
            index_to_loc_format: reader.read(),
            glyph_data_format: reader.read(),
        }
    }
}
