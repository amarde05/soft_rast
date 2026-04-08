use std::{f32::consts::PI, sync::Arc};

use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use softbuffer::Context;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{DeviceEvent, WindowEvent}, event_loop::{EventLoop, OwnedDisplayHandle}, keyboard::KeyCode, window::Window};

use anyhow::Result;

use crate::{engine::{input::Input, mesh::{Mesh, MeshRegistry}, time::Time}, render::{Renderer, camera::Camera}};

mod render;
mod engine;
mod res;

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
    time: Time,
    mesh_registry: MeshRegistry,
    mesh_rot: Vec3,
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

        let mut mesh_registry = MeshRegistry::new();
        mesh_registry.register_mesh(res::load_mesh("teapot.obj"));
        mesh_registry.register_mesh(res::load_mesh("monkey.obj"));

        Self {
            window: window.clone(),
            renderer,
            camera,
            input,
            time: Time::new(),
            mesh_registry,
            mesh_rot: Vec3::ZERO
        }
    }

    fn lock_cursor(&self) {
        self.window.set_cursor_visible(false);
        self.window.set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
    }

    fn unlock_cursor(&self) {
        self.window.set_cursor_visible(true);
        self.window.set_cursor_grab(winit::window::CursorGrabMode::None).unwrap();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.resize(size);
        self.camera.set_aspect(get_aspect(size));
    }

    fn render(&mut self) {
        self.camera.clean();
        self.renderer.render(&self.camera, &self.mesh_registry);
    }

    fn init(&mut self) {
        self.lock_cursor();
    }

    fn update(&mut self) {
        let dt = self.time.tick();

        if self.input.get_key_down(KeyCode::Escape) {
            self.unlock_cursor();
        }

        if self.input.get_mouse_button_down(winit::event::MouseButton::Left) {
            self.lock_cursor();
        }

        if self.input.get_key(KeyCode::KeyW) {
            self.camera.move_in_dir(self.camera.forward(), 2. * dt);
        }
        
        if self.input.get_key(KeyCode::KeyS) {
            self.camera.move_in_dir(-self.camera.forward(), 2. * dt);
        }

        if self.input.get_key(KeyCode::KeyA) {
            self.camera.move_in_dir(-self.camera.right(), 2. * dt);
        }
        
        if self.input.get_key(KeyCode::KeyD) {
            self.camera.move_in_dir(self.camera.right(), 2. * dt);
        }

        if self.input.get_key(KeyCode::Space) {
            self.camera.move_in_dir(self.camera.up(), 2. * dt);
        }
        
        if self.input.get_key(KeyCode::ControlLeft) {
            self.camera.move_in_dir(-self.camera.up(), 2. * dt);
        }

        let delta = self.input.get_mouse_delta();
        if delta != Vec2::ZERO {
            self.camera.rotate(Vec3::new(-delta.y, -delta.x, 0.) * 15. * dt);
        }

        self.mesh_rot += Vec3::new(180. * dt, 180. * dt, 180. * dt) * PI / 180. * 0.;

        self.renderer.add_render_object(render::RenderObject {
            mesh_id: 1,
            model_matrix: Mat4::from_rotation_translation(
                Quat::from_euler(glam::EulerRot::XYZ, self.mesh_rot.x, self.mesh_rot.y, self.mesh_rot.z),
                Vec3::new(0., 0., -5.)
            )
        });

        self.input.end_update();

        self.window.request_redraw();
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

        let mut state = State::new(&context, window);
        state.init();
        self.state = Some(state);
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

        let input = &mut state.input;

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => state.render(),
            WindowEvent::KeyboardInput { event, .. } => input.handle_key_event(event),
            WindowEvent::MouseInput { state, button, .. } => input.handle_mouse_event(state, button),
            _ => {}
        }
    }

    fn device_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        let state = match &mut self.state {
            Some(st) => st,
            None => return,
        };

        let input = &mut state.input;

        match event {
            DeviceEvent::MouseMotion { delta } => input.handle_cursor_delta(delta),
            _ => {}
        }

    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &mut self.state {
            Some(state) => state.update(),
            _ => {}
        }
    }

    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: State) {
        self.state = Some(event);
    }
}