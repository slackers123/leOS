use corelib::types::Float;

use crate::vectors::Vec2;

/// intersect two lines.
///
/// # Note:
/// This returns none if the lines are parallell
pub fn intersect_two_lines(
    p1: Vec2<Float>,
    dir1: Vec2<Float>,
    p2: Vec2<Float>,
    dir2: Vec2<Float>,
) -> Option<Vec2<Float>> {
    // adapted from: https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/

    println!("{dir1:?}, {dir2:?}");

    let a = p1;
    let b = p1 + dir1;
    let c = p2;
    let d = p2 + dir2;

    let a1 = b.y - a.y;
    let b1 = a.x - b.x;
    let c1 = a1 * (a.x) + b1 * (a.y);

    // Line CD represented as a2x + b2y = c2
    let a2 = d.y - c.y;
    let b2 = c.x - d.x;
    let c2 = a2 * (c.x) + b2 * (c.y);

    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0.0 {
        return None;
    }

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    Some(Vec2::new(x, y))
}
