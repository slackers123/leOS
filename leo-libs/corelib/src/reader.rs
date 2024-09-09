//! This module contains a generic file reader we can use to parse binary formats like ttf or png

pub trait ByteReader {
    fn read_byte(&self, data: &[u8], index: usize) -> u8;
}

/// A reader that is both generic over the type of reader (usually big or little endian) and the
/// type being read
pub struct Reader<'a, R: ByteReader> {
    pub data: &'a [u8],
    int: R,
    index: usize,
}

impl<'a> Reader<'a, BigEndianReader> {
    pub fn new_big_endian(data: &'a [u8], start_index: usize) -> Reader<'a, BigEndianReader> {
        Self {
            data,
            int: BigEndianReader,
            index: start_index,
        }
    }
}

impl<'a> Reader<'a, LittleEndianReader> {
    pub fn new_little_endian(data: &'a [u8], start_index: usize) -> Reader<'a, LittleEndianReader> {
        Self {
            data,
            int: LittleEndianReader,
            index: start_index,
        }
    }
}

impl<'a, R: ByteReader> Reader<'a, R> {
    #[inline]
    pub fn read_byte(&mut self) -> u8 {
        let dat = self.int.read_byte(&self.data, self.index);
        self.index += 1;
        dat
    }

    #[inline]
    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.index = pos;
    }

    pub fn get_pos(&mut self) -> usize {
        self.index
    }
}

pub struct BigEndianReader;

impl ByteReader for BigEndianReader {
    fn read_byte(&self, data: &[u8], index: usize) -> u8 {
        data[index]
    }
}

pub struct LittleEndianReader;

impl ByteReader for LittleEndianReader {
    fn read_byte(&self, data: &[u8], index: usize) -> u8 {
        data[data.len() - index - 1]
    }
}

pub trait Readable {
    /// implement read for your own type. You can assume the reader is big endian
    ///
    /// you also generally probably want to inline reads
    fn read(reader: &mut Reader<impl ByteReader>) -> Self;
}

macro_rules! impl_number_readable {
    ($number:ident, $lower:ident, $shift:literal) => {
        impl Readable for $number {
            fn read(reader: &mut Reader<impl ByteReader>) -> Self {
                let dat1 = reader.read::<$lower>();
                let dat2 = reader.read::<$lower>();
                (dat1 as $number) << $shift | (dat2 as $number)
            }
        }
    };
}

impl Readable for u8 {
    #[inline]
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        reader.read_byte()
    }
}

impl Readable for u16 {
    #[inline]
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        let dat1 = reader.read_byte();
        let dat2 = reader.read_byte();
        (dat1 as u16) << 8 | (dat2 as u16)
    }
}

impl_number_readable!(u32, u16, 16);
impl_number_readable!(u64, u32, 32);

impl Readable for i8 {
    #[inline]
    fn read(reader: &mut Reader<impl ByteReader>) -> Self {
        reader.read_byte() as i8
    }
}

impl_number_readable!(i16, u8, 8);
impl_number_readable!(i32, u16, 16);
impl_number_readable!(i64, u32, 32);

pub fn read_vec<T: Readable>(reader: &mut Reader<impl ByteReader>, length: usize) -> Vec<T> {
    (0..length).map(|_| reader.read::<T>()).collect()
}
