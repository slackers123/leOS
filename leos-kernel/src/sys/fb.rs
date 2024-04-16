use core::{
    slice,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    bootboot::{self, BOOTBOOT},
    common::Color,
    error::{KError, KResult},
};

#[derive(Debug, Clone, Copy)]
pub enum FBType {
    ARGB,
    BGRA,
}

#[derive(Debug, Clone, Copy)]
pub struct FBInfo {
    /// size in bytes (u8)
    pub size: u32,
    /// width in pixel (u32)
    pub width: u32,
    /// height in pixel (u32)
    pub height: u32,
    /// width in bytes (u8)
    pub scanline: u32,
    /// The framebuffer can either be ARGB or BGRA
    ///
    /// ### Note:
    /// The alpha component is never used
    pub fb_type: FBType,
}

pub struct FB {
    raw_fb: &'static mut [u32],
    pub info: FBInfo,
}

pub static mut FB_CREATED: AtomicBool = AtomicBool::new(false);

impl FB {
    pub fn new() -> KResult<Self> {
        let bootboot_r: &BOOTBOOT = unsafe { &(*(bootboot::BOOTBOOT_INFO as *const BOOTBOOT)) };

        // scanline being 0 implies there is no frame buffer
        if bootboot_r.fb_scanline == 0 {
            return Err(KError::NoFB);
        }

        // a framebuffer was already created 2 framebuffers existing would be undefined
        unsafe {
            if FB_CREATED.load(Ordering::SeqCst) {
                return Err(KError::FBCreated);
            }
        }

        let info = FBInfo {
            size: bootboot_r.fb_size / 4,
            width: bootboot_r.fb_width,
            height: bootboot_r.fb_height,
            scanline: bootboot_r.fb_scanline,
            fb_type: if bootboot_r.fb_type == 0 {
                FBType::ARGB
            } else {
                FBType::BGRA
            },
        };

        unsafe { FB_CREATED.store(true, Ordering::SeqCst) }

        let fb = bootboot::BOOTBOOT_FB;

        // SAFETY: the memory is already allocated and passed to the kernel by the bootloader
        //         and only one framebuffer can be created
        let raw_fb =
            unsafe { slice::from_raw_parts_mut::<u32>(fb as *mut u32, info.size as usize / 4) };

        Ok(Self { info, raw_fb })
    }

    pub fn put_px(&mut self, x: u32, y: u32, col: Color) -> KResult<()> {
        if x > self.info.width || y > self.info.height {
            return Err(KError::OutOfRange);
        }
        match self.info.fb_type {
            FBType::ARGB => self.put_px_raw(x, y, col.as_argb()),
            FBType::BGRA => self.put_px_raw(x, y, col.as_bgra()),
        }
        Ok(())
    }

    /// The inputs to this function are expected to be bounds checked
    /// and the col argument is expected to be of the correct format
    pub fn put_px_raw(&mut self, x: u32, y: u32, col: u32) {
        self.raw_fb[(y * self.info.width + x) as usize] = col;
    }
}
