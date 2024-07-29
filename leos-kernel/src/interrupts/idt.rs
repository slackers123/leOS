pub struct Idt([IdtEntry; 16]);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    pub ptr_low: u16,
    pub gdt_selector: SegmentSelector,
    pub options: IdtEntryOptions,
    pub ptr_middle: u16,
    pub ptr_high: u32,
    pub reserved: u32,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IdtEntryOptions(u16);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    pub const fn new(index: u16, r_pl: PrivilegeLevel) -> Self {
        Self(index << 3 | r_pl as u16)
    }

    pub fn get_index(&self) -> u16 {
        self.0 >> 3
    }

    pub fn get_rpl(&self) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.0 & 0b11)
    }
}

pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

impl PrivilegeLevel {
    pub const fn from_u16(level: u16) -> Self {
        match level {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => panic!("privilege level can only be in range 0..=3"),
        }
    }
}
