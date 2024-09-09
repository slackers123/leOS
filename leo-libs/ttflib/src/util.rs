use corelib::reader::Readable;

#[derive(Debug, Clone)]
pub struct FourByteTag(pub String);

impl Readable for FourByteTag {
    fn read(reader: &mut corelib::reader::Reader<impl corelib::reader::ByteReader>) -> Self {
        let b1 = reader.read_byte();
        let b2 = reader.read_byte();
        let b3 = reader.read_byte();
        let b4 = reader.read_byte();

        Self(String::from_utf8(vec![b1, b2, b3, b4]).unwrap())
    }
}
