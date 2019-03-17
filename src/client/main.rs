#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate cgmath;

use gfx::traits::FactoryExt;
use gfx::format::{DepthStencil, Rgba8};
use gfx::Device;
use cgmath::Matrix2;
use cgmath::Rad;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<Rgba8> = "Target0",
        transform: gfx::Global<[[f32; 2]; 2]> = "u_Transform",
    }
}

const VERT_CODE: &'static [u8] = include_bytes!("triangle.vert");
const FRAG_CODE: &'static [u8] = include_bytes!("triangle.frag");

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [-1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [1.0, -1.0], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [0.0, 1.0], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Hello Client!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let (window, mut device, mut factory, rtv, mut stv) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(
            window_builder, context, &events_loop,
        ).unwrap();
    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory.create_pipeline_simple(&VERT_CODE, &FRAG_CODE, pipe::new())
        .unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut time: f32 = 0.0;

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: rtv,
        transform: [[1.0, 0.0], [0.0, 1.0]],
    };

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = window.get_hidpi_factor();
                        window.resize(logical_size.to_physical(dpi_factor));
                        gfx_window_glutin::update_views(&window, &mut data.out, &mut stv);
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        data.transform = Matrix2::from_angle(Rad(time)).into();
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
        std::thread::sleep(std::time::Duration::from_millis(17));
        time += 1.0 / 17.0;
    }
}
