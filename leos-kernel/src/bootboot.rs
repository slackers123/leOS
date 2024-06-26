/* automatically generated by rust-bindgen */

pub const BOOTBOOT_MAGIC: &'static [u8; 5usize] = b"BOOT\0";
pub const PROTOCOL_MINIMAL: u32 = 0;
pub const PROTOCOL_STATIC: u32 = 1;
pub const PROTOCOL_DYNAMIC: u32 = 2;
pub const PROTOCOL_BIGENDIAN: u32 = 128;
pub const LOADER_BIOS: u32 = 0;
pub const LOADER_UEFI: u32 = 4;
pub const LOADER_RPI: u32 = 8;
pub const FB_ARGB: u32 = 0;
pub const FB_RGBA: u32 = 1;
pub const FB_ABGR: u32 = 2;
pub const FB_BGRA: u32 = 3;
pub const MMAP_USED: u32 = 0;
pub const MMAP_FREE: u32 = 1;
pub const MMAP_ACPI: u32 = 2;
pub const MMAP_MMIO: u32 = 3;
pub const INITRD_MAXSIZE: u32 = 16;

pub const BOOTBOOT_MMIO: u64 = 0xfffffffff8000000; /* memory mapped IO virtual address */
pub const BOOTBOOT_FB: u64 = 0xfffffffffc000000; /* frame buffer virtual address */
pub const BOOTBOOT_INFO: u64 = 0xffffffffffe00000; /* bootboot struct virtual address */
pub const BOOTBOOT_ENV: u64 = 0xffffffffffe01000; /* environment string virtual address */
pub const BOOTBOOT_CORE: u64 = 0xffffffffffe02000; /* core loadable segment start */

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MMapEnt {
    pub ptr: u64,
    pub size: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BOOTBOOT {
    pub magic: [u8; 4usize],
    pub size: u32,
    pub protocol: u8,
    pub fb_type: u8,
    pub numcores: u16,
    pub bspid: u16,
    pub timezone: i16,
    pub datetime: [u8; 8usize],
    pub initrd_ptr: u64,
    pub initrd_size: u64,
    pub fb_ptr: *mut u8,
    pub fb_size: u32,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_scanline: u32,
    pub arch: arch_union,
    pub mmap: MMapEnt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union arch_union {
    pub x86_64: arch_x86,
    pub aarch64: arch_aarch64,
    _bindgen_union_align: [u64; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arch_x86 {
    pub acpi_ptr: u64,
    pub smbi_ptr: u64,
    pub efi_ptr: u64,
    pub mp_ptr: u64,
    pub unused0: u64,
    pub unused1: u64,
    pub unused2: u64,
    pub unused3: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arch_aarch64 {
    pub acpi_ptr: u64,
    pub mmio_ptr: u64,
    pub efi_ptr: u64,
    pub unused0: u64,
    pub unused1: u64,
    pub unused2: u64,
    pub unused3: u64,
    pub unused4: u64,
}

#[doc = " Display text on screen *"]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct psf2_t {
    pub magic: u32,
    pub version: u32,
    pub headersize: u32,
    pub flags: u32,
    pub numglyph: u32,
    pub bytesperglyph: u32,
    pub height: u32,
    pub width: u32,
    pub glyphs: u8,
}

extern "C" {}
