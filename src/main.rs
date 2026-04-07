use std::sync::Arc;

use glam::Vec3;
use softbuffer::Context;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::{EventLoop, OwnedDisplayHandle}, keyboard::KeyCode, window::Window};

use anyhow::Result;

use crate::{render::{Renderer, camera::Camera}, engine::{input::Input, time::Time}};

mod render;
mod engine;

fn main() -> Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}

struct State {
    window: Arc<Window>,
    renderer: Renderer<OwnedDisplayHandle, Arc<Window>>,
    camera: Camera,
    input: Input,
    time: Time
}

impl State {
    pub fn new(context: &Context<OwnedDisplayHandle>, window: Arc<Window>) -> Self {
        let renderer = Renderer::new(context, window.clone());

        let camera = Camera::new(
            Vec3::ZERO,
            Vec3::ZERO,
            60., get_aspect(window.inner_size())
        );

        let input = Input::new();

        Self {
            window: window.clone(),
            renderer,
            camera,
            input,
            time: Time::new()
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.resize(size);
        self.camera.set_aspect(get_aspect(size));
    }

    fn render(&mut self) {
        self.renderer.render(&self.camera);
    }
}

fn get_aspect(size: PhysicalSize<u32>) -> f32 {
    size.width as f32 / size.height as f32
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
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

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
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => state.render(),
            WindowEvent::KeyboardInput { event, .. } => state.input.handle_key_event(event),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &mut self.state {
            Some(state) => {
                // Update loop
                let dt = state.time.tick();

                if state.input.get_key(KeyCode::KeyW) {
                    state.camera.translate(Vec3::new(0., 0., -2.) * dt);
                }
                
                if state.input.get_key(KeyCode::KeyS) {
                    state.camera.translate(Vec3::new(0., 0., 2.) * dt);
                }

                if state.input.get_key(KeyCode::KeyA) {
                    state.camera.translate(Vec3::new(-2., 0., 0.) * dt);
                }
                
                if state.input.get_key(KeyCode::KeyD) {
                    state.camera.translate(Vec3::new(2., 0., 0.) * dt);
                }

                if state.input.get_key(KeyCode::Space) {
                    state.camera.translate(Vec3::new(0., 2., 0.) * dt);
                }
                
                if state.input.get_key(KeyCode::ControlLeft) {
                    state.camera.translate(Vec3::new(0., -2., 0.) * dt);
                }

                if state.input.get_key(KeyCode::KeyQ) {
                    state.camera.rotate(Vec3::new(0., 2., 0.) * dt);
                }
                
                if state.input.get_key(KeyCode::KeyE) {
                    state.camera.rotate(Vec3::new(0., -2., 0.) * dt);
                }

                state.input.end_update();

                state.window.request_redraw();
            },
            _ => {}
        }
    }

    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: State) {
        self.state = Some(event);
    }
}