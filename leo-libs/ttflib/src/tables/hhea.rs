use corelib::reader::Readable;

pub struct HheaHeader {
    major_version: u16,
    minor_version: u16,
    ascender: i16,
    descender: i16,
    line_gap: i16,
    advance_width_max: u16,
    min_left_side_bearing: i16,
    min_right_side_bearing: i16,
    x_max_extent: i16,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    reserved1: i16,
    reserved2: i16,
    reserved3: i16,
    reserved4: i16,
    metric_data_format: i16,
    number_of_h_metrics: u16,
}

impl Readable for HheaHeader {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            major_version: reader.read(),
            minor_version: reader.read(),
            ascender: reader.read(),
            descender: reader.read(),
            line_gap: reader.read(),
            advance_width_max: reader.read(),
            min_left_side_bearing: reader.read(),
            min_right_side_bearing: reader.read(),
            x_max_extent: reader.read(),
            caret_slope_rise: reader.read(),
            caret_slope_run: reader.read(),
            caret_offset: reader.read(),
            reserved1: reader.read(),
            reserved2: reader.read(),
            reserved3: reader.read(),
            reserved4: reader.read(),
            metric_data_format: reader.read(),
            number_of_h_metrics: reader.read(),
        }
    }
}
