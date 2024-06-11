use log::{error, info};
use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use simple_logger::SimpleLogger;
use winit::{
    error::EventLoopError,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowLevel},
};

// This is the logical size of the window, for winit. The window will actually
// technically be 4x as many pixels as this, because of hidpi.
const WIN_SIZE: (u32, u32) = (640, 480);

// This is the logical size of the Pixels instance. This will get scaled up evenly
// to match the size of the window, which will get scaled again to match the hidpi
// factor. Confused yet?
const PIX_SIZE: (u32, u32) = (320, 240);

fn main() -> Result<(), EventLoopError> {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let window = WindowBuilder::new()
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        .with_transparent(true)
        .build(&event_loop)
        .expect("Failed to create window");
    window.set_window_level(WindowLevel::AlwaysOnTop);
    window
        .set_cursor_hittest(false)
        .expect("Failed to disable cursor hit test");

    let window_size = window.inner_size();

    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)
        .expect("Failed to create pixels");
    pixels.clear_color(Color::TRANSPARENT);

    info!("Initialization complete");

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                },
            ..
        } => {
            info!("Input from window event");
        }

        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),

        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            if let Err(e) = pixels.render() {
                error!("Failed to render pixels: {}", e);
            }
        }
        _ => (),
    });
}
