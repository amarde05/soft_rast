use std::f32::consts::PI;

use glam::{Mat4, Quat, Vec3, Vec4};


pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    fov: f32,
    aspect: f32,
    is_dirty: bool,
    vp: Mat4
}

impl Camera {
    pub fn new(position: Vec3, rotation: Vec3, fov: f32, aspect: f32) -> Self {
        let rot_quat = Camera::euler_to_quat(rotation);

        Camera {
            position,
            rotation: rot_quat,
            fov,
            aspect,
            is_dirty: false,
            vp: Camera::get_perspective(fov, aspect) * Camera::get_view(position, rot_quat),
        }
    }

    pub fn clean(&mut self) {
        match self.is_dirty {
            true => {
                self.vp = Camera::get_perspective(self.fov, self.aspect) * Camera::get_view(self.position, self.rotation);
                self.is_dirty = false;
            },
            false => {}
        }
    }

    fn set_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn vp(&self) -> Mat4 {
        self.vp
    }

    fn get_perspective(fov: f32, aspect: f32) -> Mat4 {
        Mat4::perspective_rh(fov * PI / 180., aspect, 0.1, 1000.)
    }

    fn get_view(position: Vec3, rotation: Quat) -> Mat4 {
        Mat4::from_rotation_translation(
            rotation,
            position
        ).inverse()
    }

    fn euler_to_quat(euler: Vec3) -> Quat {
        Quat::from_euler(glam::EulerRot::XYZ, euler.x, euler.y, euler.z)
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.set_dirty();
    }

    

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        self.set_dirty();
    }

    pub fn set_rotation(&mut self, rot: Vec3) {
        self.rotation = Camera::euler_to_quat(rot * PI / 180.);
        self.set_dirty();
    }

    pub fn translate(&mut self, amount: Vec3) {
        self.set_position(self.position + amount);
    }

    pub fn rotate(&mut self, amount: Vec3) {
        self.rotation *= Camera::euler_to_quat(amount * PI / 180.);
        self.set_dirty();
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn move_in_dir(&mut self, direction: Vec3, amount: f32) {
        self.translate(direction * amount);
    }
}