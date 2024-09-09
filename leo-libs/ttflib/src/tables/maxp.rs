use corelib::reader::Readable;

pub enum MaxpHeader {
    Version05(MaxpVersion05),
    Version10(MaxpVersion10),
}

impl Readable for MaxpHeader {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        let format: u32 = reader.read();
        let major = format >> 16;
        let minor = format & 0xFFFF;
        match (major, minor) {
            (0, 5) => Self::Version05(reader.read()),
            (1, 0) => Self::Version10(reader.read()),
            _ => unimplemented!("{:x}", format),
        }
    }
}

impl MaxpHeader {
    pub fn get_num_glyphs(&self) -> u16 {
        match self {
            Self::Version05(m) => m.get_num_glyphs(),
            Self::Version10(m) => m.get_num_glyphs(),
        }
    }
}

pub struct MaxpVersion05 {
    num_glyphs: u16,
}

impl MaxpVersion05 {
    fn get_num_glyphs(&self) -> u16 {
        self.num_glyphs
    }
}

impl Readable for MaxpVersion05 {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            num_glyphs: reader.read(),
        }
    }
}

pub struct MaxpVersion10 {
    num_glyphs: u16,
    max_points: u16,
    max_contours: u16,
    max_composite_points: u16,
    max_composite_contours: u16,
    max_zones: u16,
    max_twilight_points: u16,
    max_storage: u16,
    max_function_defs: u16,
    max_instruction_defs: u16,
    max_stack_elements: u16,
    max_size_of_instructions: u16,
    max_component_elements: u16,
    max_component_depth: u16,
}

impl MaxpVersion10 {
    fn get_num_glyphs(&self) -> u16 {
        self.num_glyphs
    }
}

impl Readable for MaxpVersion10 {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        Self {
            num_glyphs: reader.read(),
            max_points: reader.read(),
            max_contours: reader.read(),
            max_composite_points: reader.read(),
            max_composite_contours: reader.read(),
            max_zones: reader.read(),
            max_twilight_points: reader.read(),
            max_storage: reader.read(),
            max_function_defs: reader.read(),
            max_instruction_defs: reader.read(),
            max_stack_elements: reader.read(),
            max_size_of_instructions: reader.read(),
            max_component_elements: reader.read(),
            max_component_depth: reader.read(),
        }
    }
}
