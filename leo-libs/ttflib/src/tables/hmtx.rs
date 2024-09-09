use corelib::reader::{read_vec, ByteReader, Readable, Reader};

pub fn get_hmtx(
    reader: &mut Reader<impl ByteReader>,
    number_of_h_metrics: usize,
    num_glyphs: usize,
) -> HorizontalMetricsTable {
    let h_metrics: Vec<LongHorMetric> = read_vec(reader, number_of_h_metrics);
    let left_side_bearings: Vec<i16> = read_vec(reader, num_glyphs - number_of_h_metrics);

    return HorizontalMetricsTable {
        h_metrics,
        left_side_bearings,
    };
}

pub struct HorizontalMetricsTable {
    h_metrics: Vec<LongHorMetric>,
    left_side_bearings: Vec<i16>,
}

pub struct LongHorMetric {
    advance_width: u16,
    lsb: i16,
}

impl Readable for LongHorMetric {
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        Self {
            advance_width: reader.read(),
            lsb: reader.read(),
        }
    }
}
