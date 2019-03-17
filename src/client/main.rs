#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::format::{DepthStencil, Rgba8};
use gfx::Device;

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello Client!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let (window, mut device, mut factory, rtv, stv) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(
            window_builder, context, &events_loop,
        ).unwrap();
    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = window.get_hidpi_factor();
                        window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        encoder.clear(&rtv, CLEAR_COLOR);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
