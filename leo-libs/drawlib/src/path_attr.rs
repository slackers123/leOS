use dpilib::LUnit;
use mathlib::color::ColA;

use crate::stroking::JoinType;

#[derive(Debug)]
pub struct PathStroke {
    pub width: LUnit,
    pub color: ColA,
    pub join: JoinType,
}

#[derive(Debug)]
pub struct PathFill {
    pub color: ColA,
}
