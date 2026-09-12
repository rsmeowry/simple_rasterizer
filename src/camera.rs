use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    pub pos: Vec3,
    pub dir: Quat,
    pub fov: f32,
    pub aspect: f32
}

impl Camera {
    pub fn proj_view(&self) -> (Mat4, Mat4) {
        let view = glam::camera::rh::view::look_to_mat4(self.pos, self.dir.mul_vec3(Vec3::Z), Vec3::Y);
        let proj = glam::camera::rh::proj::directx::perspective(self.fov, self.aspect, 0.1, 1000.);
        (proj, view)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            dir: Quat::IDENTITY,
            fov: 60f32.to_radians(),
            aspect: 800. / 640.
        }
    }
}