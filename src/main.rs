mod renderer;

use renderer::*;
use winit::dpi::*;
use winit::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn main() {
    simple_logger::init().unwrap();

    let mut winit_state = WinitState::default();
    let mut hal_state = HalState::new(&winit_state.window, "New Window").unwrap();

    loop {
        let inputs = UserInput::poll_events_loop(&mut winit_state.events_loop);
        if inputs.end_requested {
            break;
        }
        if let Some((x, y)) = inputs.new_frame_size {
            hal_state.recreate_swapchain();
            continue;
        }
        if let Err(e) = render(&mut hal_state) {
            error!("Rendering Error: {:?}", e);
            break;
        }
    }
}

pub fn render(hal: &mut HalState) -> Result<(), &str> {
    match hal.draw_clear_frame([0.1, 0.5, 0.75, 1.0]) {
        Ok(_val) => Ok(()),
        Err(e) => panic!(e),
    }
}

#[derive(Debug)]
pub struct WinitState {
    pub events_loop: EventsLoop,
    pub window: Window,
}

impl WinitState {
    /// Constructs a new 'EvetnsLoop' and 'Window" pair.
    /// Use the specified title and size (if not full screen)
    pub fn new<T: Into<String>>(
        title: T,
        size: LogicalSize,
        full_screen: bool,
    ) -> Result<Self, CreationError> {
        let events_loop = EventsLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_maximized(full_screen)
            .with_dimensions(size)
            .with_resizable(true)
            .with_min_dimensions(LogicalSize {
                width: 400.0,
                height: 300.0,
            })
            .build(&events_loop);
        output.map(|window| Self {
            events_loop,
            window,
        })
    }
}

impl Default for WinitState {
    fn default() -> Self {
        Self::new(
            "New Window",
            LogicalSize {
                width: 400.0,
                height: 300.0,
            },
            false,
        )
        .expect("Could not create a window!")
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserInput {
    pub end_requested: bool,
    pub new_frame_size: Option<(f64, f64)>,
    pub new_mouse_position: Option<(f64, f64)>,
}
impl UserInput {
    pub fn poll_events_loop(events_loop: &mut EventsLoop) -> Self {
        let mut output = UserInput::default();
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => output.end_requested = true,
            Event::WindowEvent {
                event: WindowEvent::Resized(logical),
                ..
            } => {
                output.new_frame_size = Some((logical.width, logical.height));
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                output.new_mouse_position = Some((position.x, position.y));
            }
            _ => (),
        });
        output
    }
}
