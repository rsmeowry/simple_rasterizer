use std::time::Instant;
use glam::{Mat4, Quat, Vec3};
use crate::camera::Camera;
use crate::pipeline::object::AnyRenderObject;
use crate::pipeline::Pipeline;
use crate::pipeline::shader::PerObjectUniforms;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vec3,
    pub rot: Quat,
    pub scl: Vec3
}

impl Transform {
    pub fn uniforms(&self) -> PerObjectUniforms {
        PerObjectUniforms::new(Mat4::from_scale_rotation_translation(self.scl, self.rot, self.pos))
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            scl: Vec3::ONE,
            rot: Quat::IDENTITY
        }
    }
}


pub struct World<'w> {
    object_queue: Vec<(&'w Box<dyn AnyRenderObject>, Transform)>,
    begin_time: Instant,
    last_frame: Instant,
    dt: f32,
    camera: Camera,
}

impl<'w> World<'w> {
    pub fn new() -> Self {
        Self {
            object_queue: Vec::new(),
            begin_time: Instant::now(),
            last_frame: Instant::now(),
            dt: 0.,
            camera: Camera::default()
        }
    }

    pub fn begin_frame(&mut self) {
        self.object_queue.clear();
        self.dt = (Instant::now() - self.last_frame).as_secs_f32();
        self.last_frame = Instant::now();
        println!("Frame time: {}", self.dt);
    }
    
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    
    pub fn time(&self) -> f32 {
        Instant::now().duration_since(self.begin_time).as_secs_f32()
    }
    
    pub fn delta_time(&self) -> f32 {
        self.dt
    }

    pub fn render(&self, pipeline: &mut Pipeline) {
        pipeline.draw(&self.object_queue, self.camera.uniforms());
    }

    pub fn draw_object(&mut self, object: &'w Box<dyn AnyRenderObject>, tf: Transform) {
        self.object_queue.push((object, tf));
    }
}