use drawlib::line::Line2;
use drawlib::path::{ArcTo, Fill, LineTo, MoveTo, Path, QBezierTo, Stroke};
use mathlib::color::ColA;
use mathlib::types::Float;
use mathlib::vector::Vec2;
use render::Renderer;
use renderable::Renderable;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use ttflib::points_from_gt;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod render;
mod renderable;

fn main() {
    // let font = ttflib::load_font("../Poppins-Regular.ttf".into());
    // let test: Vec<char> = "Hello, World!".chars().collect();

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let mut renderer = Renderer::default();

    // let mut h_offset = 0.0;
    // for c in test.iter() {
    //     if *c == ' ' {
    //         h_offset += 800.0;
    //         continue;
    //     }
    //     if let Some((glyph_header, glyph_table, advance_width)) = font.glyhps.get(&c) {
    //         let points = points_from_gt(glyph_header, glyph_table, Vec2::new(h_offset, 0.0), 0.2);
    //         h_offset += *advance_width as Float;
    //         for cont in points {
    //             let cont_start = cont[0];
    //             let mut path = Path::new(
    //                 Stroke {
    //                     thickness: 10.0,
    //                     col: ColA::GREEN,
    //                 },
    //                 Fill {},
    //             );
    //             path.add_moveto(MoveTo {
    //                 target: cont_start.0 + Vec2 { x: 10.0, y: 10.0 },
    //             });
    //             let mut mid_spline = None;
    //             for point in cont[1..cont.len()].iter() {
    //                 // renderer.add_renderable(Renderable::Point2(Point2 {
    //                 //     c: point.0 + Vec2 { x: 10.0, y: 10.0 },
    //                 //     r: 10.0,
    //                 //     col: if point.1 { ColA::RED } else { ColA::GREEN },
    //                 // }));
    //                 if let Some(c) = mid_spline {
    //                     path.add_qbezierto(QBezierTo {
    //                         c: c + Vec2 { x: 10.0, y: 10.0 },
    //                         p1: point.0 + Vec2 { x: 10.0, y: 10.0 },
    //                     });
    //                     mid_spline = None;
    //                 } else {
    //                     if point.1 {
    //                         path.add_lineto(drawlib::path::LineTo {
    //                             target: point.0 + Vec2 { x: 10.0, y: 10.0 },
    //                         });
    //                     } else {
    //                         mid_spline = Some(point.0)
    //                     }
    //                 }
    //             }
    //             if let Some(c) = mid_spline {
    //                 path.add_qbezierto(QBezierTo {
    //                     c: c + Vec2 { x: 10.0, y: 10.0 },
    //                     p1: cont_start.0 + Vec2 { x: 10.0, y: 10.0 },
    //                 });
    //             } else {
    //                 path.add_lineto(LineTo {
    //                     target: cont_start.0 + Vec2 { x: 10.0, y: 10.0 },
    //                 });
    //             }
    //             path.build_cache();
    //             renderer.add_renderable(Renderable::Path(path));
    //         }
    //     }
    // }
    //

    renderer.add_renderable(Renderable::Line2(Line2::new(
        Vec2::new(600, 350),
        Vec2::new(750, 350),
        10,
        ColA::RED,
    )));

    let mut path = Path::new(
        Stroke {
            thickness: 10.0,
            col: ColA::WHITE,
        },
        Fill {},
    );

    path.add_moveto(MoveTo {
        target: Vec2::new(600.0, 350.0),
    });
    // a150,150 0 0,0 -150,150
    path.add_arcto(ArcTo {
        r: Vec2::new(150.0, 150.0),
        x_axis_rotation: 0.0,
        large_arc_flag: false,
        sweep_flag: false,
        target: Vec2::new(450.0, 500.0),
    });

    // path.add_closepath();
    path.build_cache();
    renderer.add_renderable(Renderable::Path(path));

    // let mut mouse_pos = Vec2::new(0.0, 0.0);
    // let mut frames = 0;
    let mut last_timer = Instant::now();
    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    window_id,
                    event:
                        WindowEvent::CursorMoved {
                            device_id: _,
                            position: _,
                        },
                } if window_id == window.id() => {
                    // mouse_pos = Vec2::new(position.x as Float, position.y as Float);
                }
                Event::WindowEvent {
                    window_id,
                    event:
                        WindowEvent::MouseInput {
                            device_id: _,
                            state: ElementState::Pressed,
                            button: MouseButton::Left,
                        },
                } if window_id == window.id() => {}
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();
                    renderer.render(&mut buffer, width, height);

                    buffer.present().unwrap();

                    // frames += 1;
                    if last_timer.elapsed().as_millis() > 1000 {
                        // println!("fps: {frames}");
                        last_timer = Instant::now();
                        // frames = 0;
                    }
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        })
        .unwrap();
}
