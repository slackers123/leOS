use corelib::types::{Float, Uint};

use crate::{aabb::AABB, equations::Equation, funcs::approx_in_range, vectors::Vec2};

pub trait HorizLineIntersect<E: Equation>
where
    Self: Sized,
{
    fn bbox(&self) -> AABB<Float>;

    fn moved_y(&self, y_off: Float) -> Self;

    fn get_equation(&self) -> E;

    fn root_filter(&self, t: &Float, test_x: Float) -> bool;

    fn isect(&self, test_point: Vec2<Float>) -> Uint {
        let bbox = self.bbox();
        if !test_point_in_range(bbox.min.y, bbox.max.y, test_point) {
            return 0;
        }

        let moved = self.moved_y(-test_point.y);

        let equation = moved.get_equation();

        let roots = equation
            .roots()
            .into_iter()
            .filter(|t| self.root_filter(t, test_point.x))
            .collect::<Vec<Float>>();

        return roots.len() as Uint;
    }
}

pub trait HlinIsect {}

fn test_point_in_range(y_min: Float, y_max: Float, test_point: Vec2<Float>) -> bool {
    approx_in_range(&test_point.y, y_min, y_max)
}
