use std::{num::NonZeroU32, sync::Arc};

use softbuffer::{Buffer, Context, Surface};
use winit::{dpi::PhysicalSize, event_loop::OwnedDisplayHandle, window::Window};


pub struct RenderContext<'a> {
    buf: Buffer<'a, OwnedDisplayHandle, Arc<Window>>
}

pub struct Renderer {
    surface: Surface<OwnedDisplayHandle, Arc<Window>>
}

impl Renderer {
    pub fn new(context: &Context<OwnedDisplayHandle>, window: Arc<Window>) -> Self {
        let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        let size = window.inner_size();
        surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();

        Self {
            surface
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();
    }

    pub fn render(&mut self) {
        let mut buffer = self.surface.buffer_mut().unwrap();
        for index in 0..(buffer.width().get() * buffer.height().get()) {
            let y = index / buffer.width().get();
            let x = index % buffer.width().get();
            let red = x % 255;
            let green = y % 255;
            let blue = (x * y) % 255;

            buffer[index as usize] = blue | (green << 8) | (red << 16);
        }

        buffer.present().unwrap();
    }
}