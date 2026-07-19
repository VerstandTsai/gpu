use std::sync::Arc;
use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use crate::{
    state::State,
    vertex::Vertex
};

pub struct App<T: Vertex> {
    shader: String,
    vertices: Vec<T>,
    window: Option<Arc<Window>>,
    state: Option<State>,
}

impl<T: Vertex> App<T> {
    pub fn new(shader: &str, vertices: Vec<T>) -> Self {
        Self {
            shader: shader.to_string(),
            vertices,
            window: None,
            state: None
        }
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<T: Vertex> ApplicationHandler for App<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() { return }
        let attrs = Window::default_attributes();
        let window = Arc::new(
            event_loop.create_window(attrs).expect("Window creation failed")
        );
        self.window = Some(window.clone());
        self.state = pollster::block_on(
            State::new(window, &self.shader, &self.vertices)
        ).ok();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        use WindowEvent::*;
        let Some(state) = &mut self.state else { return };
        match event {
            Resized(size) => state.resize(size.width, size.height),
            RedrawRequested => state.render(),
            CloseRequested => event_loop.exit(),
            _ => ()
        }
    }
}

