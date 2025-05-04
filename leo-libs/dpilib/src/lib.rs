use corelib::types::{Float, Int};

#[derive(Debug)]
pub struct LPos {
    x: Float,
    y: Float,
}

impl LPos {
    pub fn new(x: Float, y: Float) -> Self {
        Self { x, y }
    }

    pub fn to_physical(self, scaling_factor: Float) -> PPos {
        PPos::new(
            (self.x * scaling_factor).round() as Int,
            (self.y * scaling_factor).round() as Int,
        )
    }
}

#[derive(Debug)]
pub struct PPos {
    x: Int,
    y: Int,
}

impl PPos {
    pub fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }

    pub fn to_logical(self, scaling_factor: Float) -> LPos {
        LPos::new(
            self.x as Float / scaling_factor,
            self.y as Float / scaling_factor,
        )
    }
}

#[derive(Debug)]
pub struct LSize {
    width: Float,
    height: Float,
}

impl LSize {
    pub fn new(width: Float, height: Float) -> Self {
        Self { width, height }
    }

    pub fn to_physical(self, scaling_factor: Float) -> PSize {
        PSize::new(
            (self.width * scaling_factor).round() as Int,
            (self.height * scaling_factor).round() as Int,
        )
    }
}

#[derive(Debug)]
pub struct PSize {
    width: Int,
    height: Int,
}

impl PSize {
    pub fn new(width: Int, height: Int) -> Self {
        Self { width, height }
    }

    pub fn to_logical(self, scaling_factor: Float) -> LSize {
        LSize::new(
            self.width as Float / scaling_factor,
            self.height as Float / scaling_factor,
        )
    }
}

#[derive(Debug)]
pub struct LUnit {
    val: Float,
}

impl LUnit {
    pub fn new(val: Float) -> Self {
        Self { val }
    }

    pub fn to_physical(self, scaling_factor: Float) -> PUnit {
        PUnit::new((self.val * scaling_factor).round() as Int)
    }
}

#[derive(Debug)]
pub struct PUnit {
    val: Int,
}

impl PUnit {
    pub fn new(val: Int) -> Self {
        Self { val }
    }

    pub fn to_logical(self, scaling_factor: Float) -> LUnit {
        LUnit::new(self.val as Float / scaling_factor)
    }
}

#[derive(Debug)]
pub enum Pos {
    Physical(PPos),
    Logical(LPos),
}

impl Pos {
    pub fn to_logical(self, scaling_factor: Float) -> LPos {
        match self {
            Self::Physical(ppos) => ppos.to_logical(scaling_factor),
            Self::Logical(lpos) => lpos,
        }
    }

    pub fn to_physical(self, scaling_factor: Float) -> PPos {
        match self {
            Self::Physical(ppos) => ppos,
            Self::Logical(lpos) => lpos.to_physical(scaling_factor),
        }
    }
}

impl From<PPos> for Pos {
    fn from(value: PPos) -> Self {
        Self::Physical(value)
    }
}

impl From<LPos> for Pos {
    fn from(value: LPos) -> Self {
        Self::Logical(value)
    }
}

#[derive(Debug)]
pub enum Size {
    Physical(PSize),
    Logical(LSize),
}

impl Size {
    pub fn to_logical(self, scaling_factor: Float) -> LSize {
        match self {
            Self::Physical(psize) => psize.to_logical(scaling_factor),
            Self::Logical(lsize) => lsize,
        }
    }

    pub fn to_physical(self, scaling_factor: Float) -> PSize {
        match self {
            Self::Physical(psize) => psize,
            Self::Logical(lsize) => lsize.to_physical(scaling_factor),
        }
    }
}

impl From<PSize> for Size {
    fn from(value: PSize) -> Self {
        Self::Physical(value)
    }
}

impl From<LSize> for Size {
    fn from(value: LSize) -> Self {
        Self::Logical(value)
    }
}

#[derive(Debug)]
pub enum Unit {
    Physical(PUnit),
    Logical(LUnit),
}

impl Unit {
    pub fn to_logical(self, scaling_factor: Float) -> LUnit {
        match self {
            Self::Physical(punit) => punit.to_logical(scaling_factor),
            Self::Logical(lunit) => lunit,
        }
    }

    pub fn to_physical(self, scaling_factor: Float) -> PUnit {
        match self {
            Self::Physical(punit) => punit,
            Self::Logical(lunit) => lunit.to_physical(scaling_factor),
        }
    }
}

impl From<PUnit> for Unit {
    fn from(value: PUnit) -> Self {
        Self::Physical(value)
    }
}

impl From<LUnit> for Unit {
    fn from(value: LUnit) -> Self {
        Self::Logical(value)
    }
}
