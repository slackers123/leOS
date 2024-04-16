use core::fmt;

const E9_WRITER: PortWriter = PortWriter { out_port: 0xe9 };

pub struct PortWriter {
    out_port: u16,
}

impl PortWriter {
    pub fn write_byte(&self, byte: u8) {
        unsafe { outb(self.out_port, byte) }
    }

    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for PortWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Write a single byte
#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    ::core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(preserves_flags, nomem, nostack));
}

/// Read a single byte
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    ::core::arch::asm!("in al, dx", out("al") ret, in("dx") port, options(preserves_flags, nomem, nostack));
    return ret;
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    E9_WRITER.write_fmt(args).unwrap();
}

macro_rules! kprint {
    ($($arg:tt)*) => ($crate::sys::io::_print(format_args!($($arg)*)));
}
pub(crate) use kprint;

macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::sys::io::kprint!("{}\n", format_args!($($arg)*)));
}

pub(crate) use kprintln;
