use drawlib::bezier::Bezier2;
use drawlib::line::Line2;
use drawlib::tri::Tri2;
use mathlib::color::ColA;
use mathlib::types::{Float, Uint};
use mathlib::vector::Vec2;
use render::Renderer;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod render;
mod renderable;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let mut renderer = Renderer::default();

    let mut p1 = None;
    let mut p2 = None;

    let mut mouse_pos = Vec2::new(0, 0);
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
                    mouse_pos = Vec2::new(position.x as Uint, position.y as Uint);
                }
                Event::WindowEvent {
                    window_id,
                    event:
                        WindowEvent::MouseInput {
                            device_id: _,
                            state: ElementState::Pressed,
                            button: MouseButton::Left,
                        },
                } if window_id == window.id() => {
                    if p1.is_none() {
                        p1 = Some(mouse_pos);
                    } else if p2.is_none() {
                        p2 = Some(mouse_pos)
                    } else {
                        renderer.add_renderable(renderable::Renderable::Bezier2(Bezier2::new(
                            p1.unwrap(),
                            p2.unwrap(),
                            mouse_pos,
                            10,
                            ColA::WHITE,
                        )));
                        p1 = None;
                        p2 = None;
                    }
                }
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
                        println!("fps: {frames}");
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
