
use glium::{glutin, implement_vertex, Surface, uniform};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn main() {
    implement_vertex!(Vertex, position, color);
    
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertices = vec![
        Vertex { position: [0.5, 0.5], color: [1.0, 0.0, 0.0f32] }, 
        Vertex { position: [0.5, -0.5], color: [0.0, 1.0, 0.0] }, 
        Vertex { position: [-0.5, -0.5], color: [1.0, 1.0, 1.0]  },
        Vertex { position: [-0.5, 0.5], color: [0.0, 0.0, 1.0] },
    ];
    let indices = [
        0, 1, 3,
        1, 2, 3u16
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
    let program = glium::Program::from_source(&display, include_str!("../resources/shaders/vert.glsl"), include_str!("../resources/shaders/frag.glsl"), None).unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`
        t += 0.0012;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0f32],
            ],
            ourColor: [0.0 + (t+0.5)/1.4, 0.0, 0.0f32],
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
                &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
