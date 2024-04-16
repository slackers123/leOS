pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn as_argb(self) -> u32 {
        return (self.a as u32) << 24
            | (self.r as u32) << 16
            | (self.g as u32) << 8
            | (self.b as u32) << 0;
    }

    pub fn as_bgra(self) -> u32 {
        return (self.b as u32) << 24
            | (self.g as u32) << 16
            | (self.r as u32) << 8
            | (self.a as u32) << 0;
    }
}
