//! This file is originally adapted from the bootboot mykernel rust
//! example: https://gitlab.com/bztsrc/bootboot/-/blob/master/mykernel/rust/src/main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod bootboot;

mod error;
mod interrupts;
mod sys;
mod util;

use bootboot::BOOTBOOT;
use mathlib::color::ColA;

use crate::{
    sys::{
        fb::{FBType, FB},
        io::kprintln,
    },
    util::get_local_apic_id,
};

#[no_mangle]
fn _start() -> ! {
    kprintln!("Hello, World!");
    let bootboot_r: &BOOTBOOT = unsafe { &(*(bootboot::BOOTBOOT_INFO as *const BOOTBOOT)) };

    // run on a single thread for now
    // FIXME: multicore achitecture
    let cpu_id = get_local_apic_id();
    if cpu_id != bootboot_r.bspid {
        loop {}
    }
    let mut fb = FB::new().unwrap();

    let col = ColA::new(0.0, 0.6, 0.6, 1.0);

    let col = match fb.info.fb_type {
        FBType::ARGB => col.as_argb(),
        FBType::BGRA => col.as_bgra(),
    };
    for x in 0..100 as u32 {
        for y in 0..100 as u32 {
            fb.put_px_raw(x, y, col);
        }
    }

    // hang for now
    loop {}
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    kprintln!("{info:?}");
    loop {}
}
