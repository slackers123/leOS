use crate::{
    box_extend::BoxExtend,
    math::{ACol, Corners},
};

pub struct Border {
    pub radius: Corners<u32>,
    pub size: BoxExtend<u32>,
    pub col: ACol<f32>,
}
