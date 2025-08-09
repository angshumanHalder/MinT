use std::sync::Arc;

use render::state::State;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{self, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

mod render;
mod terminal;

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attr = Window::default_attributes();
        let window = event_loop.create_window(window_attr).unwrap();
        match pollster::block_on(State::new(Arc::new(window))) {
            Ok(state) => self.state = Some(state),
            Err(e) => {
                eprintln!("Failed to initialize application state: {e}");
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button is clicked!");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.render();
            }
            _ => {}
        }
    }
}

fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn main() {
    let _ = run();
}
