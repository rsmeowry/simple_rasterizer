use crate::camera::Camera;
use crate::pipeline::object::AnyRenderObject;
use crate::pipeline::shader::PerObjectUniforms;
use crate::pipeline::{Pipeline, Uniforms};
use glam::{Mat4, Quat, Vec3};
use image::{ImageBuffer, ImageFormat, RgbImage};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vec3,
    pub rot: Quat,
    pub scl: Vec3,
}

impl Transform {
    pub fn uniforms(&self) -> PerObjectUniforms {
        PerObjectUniforms::new(Mat4::from_scale_rotation_translation(
            self.scl, self.rot, self.pos,
        ))
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            scl: Vec3::ONE,
            rot: Quat::IDENTITY,
        }
    }
}

pub struct World<'w> {
    #[allow(clippy::borrowed_box)]
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
            camera: Camera::default(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.object_queue.clear();
        self.dt = (Instant::now() - self.last_frame).as_secs_f32();
        self.last_frame = Instant::now();
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
        let (proj, view) = self.camera.proj_view();
        pipeline.draw(
            &self.object_queue,
            Uniforms::new(view, proj, self.camera.pos),
        );
    }

    pub fn render_to_png<P: AsRef<Path>>(&mut self, pipeline: &mut Pipeline, out: P) {
        let (old_w, old_h) = {
            let buf_m = pipeline.buffer_mut();
            let o = (buf_m.width(), buf_m.height());
            buf_m.resize(1920, 1080);
            o
        };
        self.camera.aspect = 1920. / 1080.;
        let (proj, view) = self.camera.proj_view();
        pipeline.draw(&self.object_queue, Uniforms::new(view, proj, self.camera.pos));

        let pixels = unpack_col_buf(pipeline.buffer().color());
        let image: RgbImage = ImageBuffer::from_raw(1920, 1080, pixels).unwrap();
        image.save_with_format(out, ImageFormat::Jpeg).unwrap();

        pipeline.buffer_mut().resize(old_w, old_h);
        self.camera.aspect = old_w as f32 / old_h as f32;
    }

    #[allow(clippy::borrowed_box)]
    pub fn draw_object(&mut self, object: &'w Box<dyn AnyRenderObject>, tf: Transform) {
        self.object_queue.push((object, tf));
    }
}

impl<'w> Default for World<'w> {
    fn default() -> Self {
        Self::new()
    }
}

fn unpack_col_buf(buf: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(buf.len() * 3);

    for &pixel in buf {
        out.push(((pixel >> 16) & 0xff) as u8); // R
        out.push(((pixel >> 8) & 0xff) as u8);  // G
        out.push((pixel & 0xff) as u8);         // B
    }

    out
}