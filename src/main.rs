use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Copied from winit examples
mod fill;

fn main() {
    pollster::block_on(run_event_loop());
}

async fn run_event_loop() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut cursor_moved = false;

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if cursor_moved {
                fill::fill_window(&window, 0xFFFF1818);
                cursor_moved = false;
            } else {
                fill::fill_window(&window, 0xFF181818);
            }
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CursorMoved { .. } => {
                cursor_moved = true;
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        _ => (),
    });
}
