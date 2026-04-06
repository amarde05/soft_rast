use std::{num::NonZeroU32, sync::Arc};

use softbuffer::{Context, Surface};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{EventLoop, OwnedDisplayHandle}, window::Window};

use anyhow::Result;

use crate::render::Renderer;

mod render;

fn main() -> Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}

struct State {
    window: Arc<Window>,
    renderer: Renderer,
}

impl State {
    pub fn new(context: &Context<OwnedDisplayHandle>, window: Arc<Window>) -> Self {
        let renderer = Renderer::new(context, window.clone());

        Self {
            window: window.clone(),
            renderer
        }
    }
}

struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();        
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

        self.state = Some(State::new(&context, window));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(st) => st,
            None => return,
        };

        if window_id != state.window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                state.renderer.resize(size);
            },
            WindowEvent::RedrawRequested => {
                state.renderer.render();
            },
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: State) {
        self.state = Some(event);
    }
}