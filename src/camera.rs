use glam::{Mat4, Quat, Vec3};
use crate::pipeline::Uniforms;

pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub fov: f32,
    pub aspect: f32
}

impl Camera {
    pub fn uniforms(&self) -> Uniforms {
        let view = glam::camera::rh::view::look_to_mat4(self.pos,  self.dir, Vec3::Y);
        let proj = glam::camera::rh::proj::directx::perspective(self.fov, self.aspect, 0.1, 1000.);
        Uniforms::new(view, proj)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            dir: Vec3::Z,
            fov: 60f32.to_radians(),
            aspect: 800. / 640.
        }
    }
}