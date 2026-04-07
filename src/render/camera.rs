use std::f32::consts::PI;

use glam::{Mat4, Quat, Vec3, Vec4};


pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    fov: f32,
    perspective: Mat4,
    view: Mat4
}

impl Camera {
    pub fn new(position: Vec3, rotation: Vec3, fov: f32, aspect: f32) -> Self {
        Camera {
            position,
            rotation,
            fov,
            perspective: Camera::get_perspective(fov, aspect),
            view: Camera::get_view(position, rotation)
        }
    }

    fn get_perspective(fov: f32, aspect: f32) -> Mat4 {
        Mat4::perspective_rh(fov * PI / 180., aspect, 0.1, 1000.)
    }

    fn get_view(position: Vec3, rotation: Vec3) -> Mat4 {
        Mat4::from_rotation_translation(
            Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z),
            position
        ).inverse()
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.perspective = Camera::get_perspective(self.fov, aspect);
    }

    pub fn project(&self, point: Vec4) -> Vec4 {
        self.perspective * self.view * point
    }

    fn update_view_matrix(&mut self) {
        self.view = Camera::get_view(self.position, self.rotation);
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        self.update_view_matrix();
    }

    pub fn set_rotation(&mut self, rot: Vec3) {
        self.rotation = rot;
        self.update_view_matrix();
    }

    pub fn translate(&mut self, amount: Vec3) {
        self.set_position(self.position + amount);
    }

    pub fn rotate(&mut self, amount: Vec3) {
        self.set_rotation(self.rotation + amount);
    }
}