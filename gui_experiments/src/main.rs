use drawlib::path::{CBezierTo, Fill, LineTo, MoveTo, Path, QBezierTo, Stroke};
use drawlib::point::Point2;
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
    let font = ttflib::load_font("../Poppins-Regular.ttf".into());
    let test: Vec<char> = "Hello World!".chars().collect();

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let mut renderer = Renderer::default();

    let mut h_offset = 0.0;
    for c in test.iter() {
        if let Some((glyph_header, glyph_table, advance_width)) = font.glyhps.get(&c) {
            let points = points_from_gt(glyph_header, glyph_table, Vec2::new(h_offset, 0.0), 0.2);
            h_offset += *advance_width as Float;
            for point in &points {
                renderer.add_renderable(Renderable::Point2(Point2 {
                    c: *point + Vec2 { x: 10.0, y: 10.0 },
                    r: 10.0,
                    col: ColA::RED,
                }))
            }
            let mut i = 0;
            for c_end in &glyph_table.end_pts_of_contours {
                let mut path = Path::new(
                    Stroke {
                        thickness: 10.0,
                        col: ColA::GREEN,
                    },
                    Fill {},
                );
                path.add_moveto(MoveTo {
                    target: points[i] + Vec2 { x: 10.0, y: 10.0 },
                });
                for idx in (i + 1)..=*c_end as usize {
                    path.add_lineto(drawlib::path::LineTo {
                        target: points[idx as usize] + Vec2 { x: 10.0, y: 10.0 },
                    })
                }
                path.add_lineto(LineTo {
                    target: points[i] + Vec2 { x: 10.0, y: 10.0 },
                });
                path.build_cache();
                renderer.add_renderable(Renderable::Path(path));
                i = *c_end as usize + 1;
            }
        }
    }

    let mut mouse_pos = Vec2::new(0.0, 0.0);
    let mut frames = 0;
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
                            position,
                        },
                } if window_id == window.id() => {
                    mouse_pos = Vec2::new(position.x as Float, position.y as Float);
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

                    frames += 1;
                    if last_timer.elapsed().as_millis() > 1000 {
                        // println!("fps: {frames}");
                        last_timer = Instant::now();
                        frames = 0;
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
