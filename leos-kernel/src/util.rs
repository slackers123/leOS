use core::arch::asm;

pub fn get_local_apic_id() -> u16 {
    let res: u64;
    unsafe { asm!("mov eax, 1", "cpuid", "mov eax, ebx", out("eax") res) }
    (res >> 24) as u16
}
