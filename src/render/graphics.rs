use glam::{Vec4, Vec4Swizzles};
use softbuffer::Buffer;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

#[derive(Clone, Copy)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32,
}

impl Color {
    pub const RED: Color = Color { r: 255, g: 0, b: 0};
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0};
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255};

    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Color {
            r: r % 255,
            g: g % 255,
            b: b % 255,
        }
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.b | (value.g << 8) | (value.r << 16)
    }
}

pub struct Graphics<'a, D, W> {
    buf: Buffer<'a, D, W>
}

impl<'a, D: HasDisplayHandle, W: HasWindowHandle> Graphics<'a, D, W> {
    pub fn new(buf: Buffer<'a, D, W>) -> Self {
        Self {
            buf
        }
    }

    pub fn present(self) {
        self.buf.present().unwrap();
    }

    pub fn width(&self) -> u32 {
        self.buf.width().get()
    }

    pub fn height(&self) -> u32 {
        self.buf.height().get()
    }


    fn get_idx(&self, x: u32, y: u32) -> usize {
        (x + y * self.buf.width().get()) as usize
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.buf.width().get() as i32 && y >= 0 && y < self.buf.height().get() as i32
    }

    // Sets the pixel at (x,y) to color if it is in bounds
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        match self.in_bounds(x, y) {
            true => {
                let idx = self.get_idx(x as u32, y as u32);
                self.buf[idx] = color.into();
            },
            false => {}
        }        
    }

    // Copys cols to the rectangle whose top-left corner is (x,y)
    // The length of cols must be w * h
    pub fn set_pixels(&mut self, x: u32, y: u32, w: u32, h: u32, cols: &[u32]) {
        for i in y..h {
            let idx = self.get_idx(x, i);
            self.buf[idx..idx + w as usize].copy_from_slice(&cols[idx..idx + w as usize]);
        }
    }

    // Draws a rectangle whose top-left corner is (x,y)
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        let cols: Vec<u32> = vec![color.into(); w as usize];

        for i in y..h {
            let idx = self.get_idx(x, i);
            self.buf[idx..idx + w as usize].copy_from_slice(&cols);
        }
    }

    // Clears the buffer with the given color
    pub fn clear(&mut self, color: Color) {
        let cols: Vec<u32> = vec![color.into(); (self.width() * self.height()) as usize];
        self.buf.copy_from_slice(&cols);
    }

    // Draws a line from a to b
    pub fn draw_line(&mut self, a: Point, b: Point, color: Color) {
        let dx = (b.x - a.x).abs();
        let sx = match a.x < b.x {
            true => 1,
            false => -1
        };

        let dy = -(b.y - a.y).abs();
        let sy = match a.y < b.y {
            true => 1,
            false => -1
        };

        let mut error = dx + dy;

        let mut x = a.x;
        let mut y = a.y;

        loop {
            self.set_pixel(x, y, color);

            let e2 = 2 * error;
            if e2 >= dy {
                if x == b.x {
                    break;
                }

                error += dy;
                x += sx;
            }
            
            if e2 <= dx {
                if y == b.y {
                    break;
                }

                error += dx;
                y += sy;
            }
        }
    }

    // Draws a horizontal line between x0 and x1 and y
    fn draw_line_horizontal(&mut self, x0: i32, x1: i32, y: i32, color: Color) {
        if x0 < x1 {
            for x in x0..x1+1 {
                self.set_pixel(x, y, color);
            }
        }
        else {
            for x in x1..x0+1 {
                self.set_pixel(x, y, color);
            }
        }
    }

    // Draws a triangle with a flat bottom
    // Assumes v2.y == v3.y
    fn draw_triangle_bf(&mut self, v1: Point, v2: Point, v3: Point, color: Color) {
        let mut v1v2 = LineIter::new(v1, v2);
        let mut v1v3 = LineIter::new(v1, v3);

        let mut current_y = v1.y;

        let mut p1: Point = v1;
        let mut p2: Point = v1;

        while current_y < v2.y {
            let mut max_x = p1.x.max(p2.x);
            let mut min_x = p1.x.min(p2.x);

            while let Some(new_p1) = v1v2.next() {
                p1 = new_p1;

                if new_p1.y != current_y {
                    break;
                }

                max_x = max_x.max(new_p1.x);
                min_x = min_x.min(new_p1.x);
            }

            while let Some(new_p2) = v1v3.next() {
                p2 = new_p2;

                if new_p2.y != current_y {
                    break;
                }

                max_x = max_x.max(new_p2.x);
                min_x = min_x.min(new_p2.x);
            }

            self.draw_line_horizontal(min_x, max_x, current_y, color);

            current_y = p1.y;
        }

        self.draw_line_horizontal(v2.x, v3.x, v2.y, color);
    }

    // Draws a triangle with a flat top
    // Assumes v1.y == v2.y
    fn draw_triangle_tf(&mut self, v1: Point, v2: Point, v3: Point, color: Color) {
        let mut v3v1 = LineIter::new(v3, v1);
        let mut v3v2 = LineIter::new(v3, v2);

        let mut current_y = v3.y;

        let mut p1: Point = v3;
        let mut p2: Point = v3;

        while current_y > v2.y {
            let mut max_x = p1.x.max(p2.x);
            let mut min_x = p1.x.min(p2.x);

            while let Some(new_p1) = v3v1.next() {
                p1 = new_p1;

                if new_p1.y != current_y {
                    break;
                }

                max_x = max_x.max(new_p1.x);
                min_x = min_x.min(new_p1.x);
            }

            while let Some(new_p2) = v3v2.next() {
                p2 = new_p2;

                if new_p2.y != current_y {
                    break;
                }

                max_x = max_x.max(new_p2.x);
                min_x = min_x.min(new_p2.x);
            }

            self.draw_line_horizontal(min_x, max_x, current_y, color);

            current_y = p1.y;
        }

        self.draw_line_horizontal(v1.x, v2.x, v2.y, color);
    }

    // Draws a triangle given 3 Points
    pub fn draw_triangle_direct(&mut self, verts: &[Point; 3], color: Color) {
        let mut sorted_verts = verts.clone();
        sorted_verts.sort_by(|a, b| a.y.cmp(&b.y));
        
        if sorted_verts[1].y == sorted_verts[2].y {
            self.draw_triangle_bf(sorted_verts[0], sorted_verts[1], sorted_verts[2], color);
        }
        else if sorted_verts[0].y == sorted_verts[1].y {
            self.draw_triangle_tf(sorted_verts[0], sorted_verts[1], sorted_verts[2], color);
        }
        else {
            let dx3: f32 = (sorted_verts[2].x - sorted_verts[0].x) as f32;
            let dy2: f32 = (sorted_verts[1].y - sorted_verts[0].y) as f32;
            let dy3: f32 = (sorted_verts[2].y - sorted_verts[0].y) as f32;

            let v4 = Point {
                x: sorted_verts[0].x + (dx3 * dy2 / dy3) as i32,
                y: sorted_verts[1].y
            };

            self.draw_triangle_bf(sorted_verts[0], sorted_verts[1], v4, color);
            self.draw_triangle_tf(sorted_verts[1], v4, sorted_verts[2], color);
        }
    }

    fn get_point_from_clip(&self, clip: Vec4) -> Point {
        let ndc = clip.xyz() / clip.w;

        let screen_x = ((ndc.x + 1.) * 0.5 * self.width() as f32) as i32;
        let screen_y = ((ndc.y + 1.) * 0.5 * self.height() as f32) as i32;

        Point { x: screen_x, y: screen_y}
    }

    // Draws a triangle whose vertices are in clip space [-1, 1]
    pub fn draw_triangle_clip(&mut self, verts: &[Vec4; 3], color: Color) {
        let points = [
            self.get_point_from_clip(verts[0]),
            self.get_point_from_clip(verts[1]),
            self.get_point_from_clip(verts[2])
        ];

        self.draw_triangle_direct(&points, color);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub struct LineIter {
    pub a: Point,
    pub b: Point,
    
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
    error: i32,
    x: i32,
    y: i32,
    prev_x: i32,
    prev_y: i32,
}

impl LineIter {
    pub fn new(a: Point, b: Point) -> Self {
        let dx = (b.x - a.x).abs();
        let sx = match a.x < b.x {
            true => 1,
            false => -1
        };

        let dy = -(b.y - a.y).abs();
        let sy = match a.y < b.y {
            true => 1,
            false => -1
        };

        LineIter { 
            a, b, 
            dx, dy, sx, sy, 
            error: dx + dy, 
            x: a.x, y: a.y ,
            prev_x: a.x, prev_y: a.y
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.prev_x == self.b.x && self.prev_y == self.b.y {
            return None;
        }

        let last_point = Point { x: self.x, y: self.y };

        self.prev_x = last_point.x;
        self.prev_y = last_point.y;

        let e2 = 2 * self.error;
        if e2 >= self.dy {
            if self.x == self.b.x {
                return Some(last_point);
            }

            self.error += self.dy;
            self.x += self.sx;
        }
        
        if e2 <= self.dx {
            if self.y == self.b.y {
                return Some(last_point);
            }

            self.error += self.dx;
            self.y += self.sy;
        }

        Some(last_point)
    }
}