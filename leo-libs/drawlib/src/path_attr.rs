use dpilib::LUnit;
use mathlib::color::ColA;

use crate::stroking::JoinType;

#[derive(Debug, Clone)]
pub struct PathStroke {
    pub width: LUnit,
    pub color: ColA,
    pub join: JoinType,
}

#[derive(Debug, Clone)]
pub struct PathFill {
    pub color: ColA,
}

#[derive(Debug, Clone)]
pub struct PathAttrs {
    pub stroke: PathStroke,
    pub fill: PathFill,
}
