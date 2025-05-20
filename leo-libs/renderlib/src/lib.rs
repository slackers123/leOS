use corelib::types::{Float, Uint};
use image::{ImageBuffer, Rgb, RgbImage};
use mathlib::vectors::Vec2;
use primitive::{MeshType, Primitive};

pub mod material;
pub mod primitive;

pub fn draw_primitives(prims: &[Primitive]) -> RgbImage {
    let mut img = RgbImage::new(1000, 1000);
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

                    let dims = img.dimensions();

                    let min_x = min_x.clamp(0.0, (dims.0 - 1) as Float);
                    let max_x = max_x.clamp(0.0, (dims.0 - 1) as Float);
                    let min_y = min_y.clamp(0.0, (dims.1 - 1) as Float);
                    let max_y = max_y.clamp(0.0, (dims.1 - 1) as Float);

                    for y in min_y as Uint..=max_y as Uint {
                        for x in min_x as Uint..=max_x as Uint {
                            let p = Vec2::new(x as Float, y as Float);

                            let edge1 = (trib.x - tria.x) * (p.y - tria.y)
                                - (trib.y - tria.y) * (p.x - tria.x);
                            let edge2 = (tric.x - trib.x) * (p.y - trib.y)
                                - (tric.y - trib.y) * (p.x - trib.x);
                            let edge3 = (tria.x - tric.x) * (p.y - tric.y)
                                - (tria.y - tric.y) * (p.x - tric.x);

                            if edge1 >= 0.0 && edge2 >= 0.0 && edge3 >= 0.0 {
                                img.put_pixel(x, y, Rgb([255, 255, 255]));
                            }
                        }
                    }
                }
            }
            _ => todo!("other mesh types"),
        }
    }
    img
}
