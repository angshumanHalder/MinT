use std::sync::Arc;

use wgpu::{Device, Queue, Surface};
use winit::window::Window;

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        Ok(Self { window })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        // TODO: Add logic later
    }

    pub fn render(&mut self) {
        self.window.request_redraw();
        // TODO: add more stuffs
    }
}
