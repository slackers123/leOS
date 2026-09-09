use std::num::NonZeroUsize;

use corelib::types::Float;
use imglib::{RgbaF32, RgbaF32Image, RgbaU8};
// use image::{ImageBuffer, Rgb, RgbImage};
use mathlib::vectors::Vec2;
use primitive::{MeshType, Primitive};

pub mod material;
pub mod primitive;

pub const MSAA_COUNT: NonZeroUsize = NonZeroUsize::new(4).unwrap();

pub fn draw_primitives(prims: &[Primitive], target: &mut RgbaF32Image) {
    for primitive in prims {
        match primitive.mesh.ty {
            MeshType::Triangle => {
                for tri in primitive.mesh.indices.chunks(3) {
                    let a = primitive.mesh.vertices[tri[0]];
                    let b = primitive.mesh.vertices[tri[1]];
                    let c = primitive.mesh.vertices[tri[2]];

                    let cross_product = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

                    let tria = a;
                    let mut trib = b;
                    let mut tric = c;

                    if cross_product < 0.0 {
                        // Swap b and c to make the order clockwise
                        let temp = trib;
                        trib = tric;
                        tric = temp;
                    }

                    let min_x = tria.x.min(trib.x).min(tric.x);
                    let max_x = tria.x.max(trib.x).max(tric.x);
                    let min_y = tria.y.min(trib.y).min(tric.y);
                    let max_y = tria.y.max(trib.y).max(tric.y);

                    let dims = target.dimensions();

                    let min_x = min_x.clamp(0.0, (dims.0 - 1) as Float);
                    let max_x = max_x.clamp(0.0, (dims.0 - 1) as Float);
                    let min_y = min_y.clamp(0.0, (dims.1 - 1) as Float);
                    let max_y = max_y.clamp(0.0, (dims.1 - 1) as Float);

                    for y in min_y as usize..=max_y as usize {
                        for x in min_x as usize..=max_x as usize {
                            let mut hit_cnt = 0;
                            for i in 0..MSAA_COUNT.get() {
                                let base = MSAA_COUNT.get().isqrt();

                                let x_off = (i % base) as Float / base as Float;
                                let y_off = (i / base) as Float / base as Float;

                                let p = Vec2::new(x as Float + x_off, y as Float + y_off);

                                let edge1 = (trib.x - tria.x) * (p.y - tria.y)
                                    - (trib.y - tria.y) * (p.x - tria.x);
                                let edge2 = (tric.x - trib.x) * (p.y - trib.y)
                                    - (tric.y - trib.y) * (p.x - trib.x);
                                let edge3 = (tria.x - tric.x) * (p.y - tric.y)
                                    - (tria.y - tric.y) * (p.x - tric.x);

                                if edge1 >= 0.0 && edge2 >= 0.0 && edge3 >= 0.0 {
                                    hit_cnt += 1;
                                }
                            }

                            if hit_cnt != 0 {
                                if hit_cnt == MSAA_COUNT.get() {
                                    let col = primitive.material.get_color();
                                    target.put_pixel(x, y, RgbaF32::from_cola(col));
                                } else {
                                    let hit_share = hit_cnt as Float / MSAA_COUNT.get() as Float;

                                    let mut top_col = primitive.material.get_color();
                                    top_col.a *= hit_share;

                                    let bottom_col = target.get_pixel(x, y);

                                    let a_out = top_col.a + bottom_col.a * (1. - top_col.a);

                                    let r_out = (top_col.r * top_col.a
                                        + bottom_col.r * bottom_col.a * (1. - top_col.a))
                                        / a_out;

                                    let g_out = (top_col.g * top_col.a
                                        + bottom_col.g * bottom_col.a * (1. - top_col.a))
                                        / a_out;

                                    let b_out = (top_col.b * top_col.a
                                        + bottom_col.b * bottom_col.a * (1. - top_col.a))
                                        / a_out;

                                    target.put_pixel(
                                        x,
                                        y,
                                        RgbaF32 {
                                            r: r_out,
                                            g: g_out,
                                            b: b_out,
                                            a: (top_col.a + bottom_col.a).clamp(0., 1.),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
            _ => todo!("other mesh types"),
        }
    }
}
