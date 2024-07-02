use mathlib::color::ColA;

use crate::{box_extend::BoxExtend, math::Corners};

pub struct Border {
    pub radius: Corners<u32>,
    pub size: BoxExtend<u32>,
    pub col: ColA,
}
