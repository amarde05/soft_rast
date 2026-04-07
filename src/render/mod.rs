use std::num::NonZeroU32;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use softbuffer::{Context, Surface};
use winit::{dpi::PhysicalSize, raw_window_handle::{HasDisplayHandle, HasWindowHandle}};

use crate::{engine::mesh::MeshRegistry, render::{camera::Camera, graphics::{Color, Graphics, Point}, pipeline::Pipeline}};

mod graphics;
mod pipeline;
pub(crate) mod camera;

pub struct Renderer<D, W> {
    surface: Surface<D, W>,
    render_objects: Vec<RenderObject>,
    pipeline: Pipeline
}

impl<D: HasDisplayHandle, W: HasWindowHandle> Renderer<D, W> {
    pub fn new(context: &Context<D>, window: W) -> Self {
        let surface = softbuffer::Surface::new(context, window).unwrap();

        Self {
            surface,
            render_objects: Vec::new(),
            pipeline: Pipeline::new()
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();
    }

    pub fn render(&mut self, camera: &Camera, mesh_registry: &MeshRegistry) {
        let tris = self.pipeline.submit_ros(camera, &self.render_objects, mesh_registry);
        self.render_objects .clear();

        let mut g = Graphics::new(self.surface.buffer_mut().unwrap());

        g.clear(Color::RED);

        for tri in tris {
            g.draw_triangle_clip(&tri.verts, Color::BLUE, graphics::DrawMode::Fill);
        }

        g.present();
    }

    pub fn add_render_object(&mut self, render_object: RenderObject) {
        self.render_objects.push(render_object);
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub verts: [Vec4; 3]
}

impl Triangle {
    pub fn new(verts: [Vec4; 3]) -> Self {
        Triangle { 
            verts: verts.iter().map(|v| Vec4::new(v.x, v.y, v.z, 1.0)).collect::<Vec<Vec4>>().try_into().unwrap()
        }
    }

    pub fn get_normal(&self) -> Vec3 {
        let edge1 = (self.verts[1] - self.verts[0]).xyz();
        let edge2 = (self.verts[2] - self.verts[0]).xyz();
        edge1.cross(edge2)
    }
}

#[derive(Clone)]
pub struct RenderObject {
    pub mesh_id: usize,
    pub model_matrix: Mat4,
}