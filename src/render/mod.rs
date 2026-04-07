use std::num::NonZeroU32;

use glam::Vec4;
use softbuffer::{Context, Surface};
use winit::{dpi::PhysicalSize, raw_window_handle::{HasDisplayHandle, HasWindowHandle}};

use crate::render::graphics::{Color, Graphics, Point};

use crate::camera::Camera;

mod graphics;

pub struct Renderer<D, W> {
    surface: Surface<D, W>,
}

impl<D: HasDisplayHandle, W: HasWindowHandle> Renderer<D, W> {
    pub fn new(context: &Context<D>, window: W) -> Self {
        let surface = softbuffer::Surface::new(context, window).unwrap();

        Self {
            surface,
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

    pub fn render(&mut self, camera: &Camera) {
        let mut g = Graphics::new(self.surface.buffer_mut().unwrap());

        g.clear(Color::RED);

        g.draw_line(Point::new(0, 0), Point::new(g.width() as i32 - 1, g.height() as i32 - 1), Color::BLUE);

        let v1 = Vec4::new(0., 0.0, -2.0, 1.0);
        let v2 = Vec4::new(0.25, 0.25, -2.0, 1.0);
        let v3 = Vec4::new(-0.25, 0.25, -2.0, 1.0);

        //g.draw_triangle_direct(&[Point::new(0, 0), Point::new(0, g.height() as i32 - 1), Point::new(g.width() as i32 - 1, g.height() as i32 - 1)], Color::GREEN);

        g.draw_triangle_clip(&[
            camera.project(v1),
            camera.project(v2),
            camera.project(v3)
        ], Color::GREEN);

        g.present();
    }
}