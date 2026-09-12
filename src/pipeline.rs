use crate::framebuffer::{Framebuffer, col3_to_u32};
use crate::pipeline::object::AnyRenderObject;
use crate::tri::Tri;
use crate::world::Transform;
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use std::any::Any;

pub mod object;
pub mod shader;
pub mod vertex;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Color(Vec3);

#[derive(Debug, Clone)]
pub struct Uniforms {
    camera_pos: Vec3,
    #[allow(dead_code)]
    view_dir: Vec3,
    view: Mat4,
    proj: Mat4,
}

impl Uniforms {
    pub fn new(view: Mat4, proj: Mat4, camera_pos: Vec3) -> Self {
        Self {
            view,
            proj,
            view_dir: view.transform_vector3(Vec3::Z),
            camera_pos,
        }
    }

    pub fn view(&self) -> &Mat4 {
        &self.view
    }

    pub fn proj(&self) -> &Mat4 {
        &self.proj
    }
}

pub struct Pipeline {
    fb: Framebuffer,
}

impl Pipeline {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            fb: Framebuffer::new(width, height),
        }
    }

    pub fn buffer(&self) -> &Framebuffer {
        &self.fb
    }

    #[allow(clippy::borrowed_box)]
    pub fn draw(
        &mut self,
        objects: &Vec<(&Box<dyn AnyRenderObject>, Transform)>,
        uniforms: Uniforms,
    ) {
        self.fb.clear(0x7fc7f4);
        self.fb.clear_depth();
        for (obj, tf) in objects {
            // vertex stage
            let obj_uniforms = tf.uniforms();
            let tf_vertices: Vec<(Vec4, Box<dyn Any>)> = obj
                .vertices()
                .iter()
                .map(|v| obj.do_vertex_stage(v, &uniforms, &obj_uniforms))
                .collect();

            // rasterizing
            for tri in obj.indices().chunks(3) {
                let (c0, v0): &(Vec4, Box<dyn Any>) = &tf_vertices[tri[0] as usize];
                let (c1, v1): &(Vec4, Box<dyn Any>) = &tf_vertices[tri[1] as usize];
                let (c2, v2): &(Vec4, Box<dyn Any>) = &tf_vertices[tri[2] as usize];

                // behind camera clipping
                if c0.w <= 0. || c1.w <= 0. || c2.w <= 0. {
                    continue;
                }

                // perspective divide, normalize coordinates, viewport transform, screen space
                let w = self.fb.width() as f32;
                let h = self.fb.height() as f32;
                let s0 = to_screen(*c0, w, h);
                let s1 = to_screen(*c1, w, h);
                let s2 = to_screen(*c2, w, h);

                let tri2d = Tri(s0.pos, s1.pos, s2.pos);
                if tri2d.area() <= 0. {
                    continue; // facing the other way
                }

                self.rasterize_triangle(
                    obj.as_ref(),
                    &tri2d,
                    [s0, s1, s2],
                    [v0, v1, v2],
                    &uniforms,
                );
            }
        }
    }

    fn rasterize_triangle(
        &mut self,
        obj: &dyn AnyRenderObject,
        tri: &Tri,
        sv: [ScreenVert; 3],
        vars: [&Box<dyn Any>; 3],
        uniforms: &Uniforms,
    ) {
        let min_x = sv
            .iter()
            .map(|v| v.pos.x)
            .fold(f32::INFINITY, f32::min)
            .floor()
            .max(0.0) as usize;
        let max_x = sv
            .iter()
            .map(|v| v.pos.x)
            .fold(f32::NEG_INFINITY, f32::max)
            .ceil()
            .min(self.fb.width() as f32) as usize;
        let min_y = sv
            .iter()
            .map(|v| v.pos.y)
            .fold(f32::INFINITY, f32::min)
            .floor()
            .max(0.0) as usize;
        let max_y = sv
            .iter()
            .map(|v| v.pos.y)
            .fold(f32::NEG_INFINITY, f32::max)
            .ceil()
            .min(self.fb.height() as f32) as usize;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let sample = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                // testing if inside a triangle
                let (w0, w1, w2) = tri.barycentric_weights(sample);
                if w0 < 0. || w1 < 0. || w2 < 0. {
                    continue;
                }

                // getting depth
                let depth = w0 * sv[0].ndc_z + w1 * sv[1].ndc_z + w2 * sv[2].ndc_z;

                // depth test !!!!!
                if depth >= self.fb.depth_at(x, y) {
                    continue;
                }

                // perspective correct weights
                let iw = w0 * sv[0].inv_w + w1 * sv[1].inv_w + w2 * sv[2].inv_w;
                let pw = (
                    w0 * sv[0].inv_w / iw,
                    w1 * sv[1].inv_w / iw,
                    w2 * sv[2].inv_w / iw,
                );

                // interpolation + fs
                let interp = obj.interpolate_varyings(
                    vars[0].as_ref(),
                    vars[1].as_ref(),
                    vars[2].as_ref(),
                    pw,
                );
                let col = obj.do_frag_stage(interp, uniforms);

                self.fb.set_col(x, y, col3_to_u32(col.xyz()));
                self.fb.set_depth(x, y, depth);
            }
        }
    }
}

struct ScreenVert {
    pos: Vec2,  // pixel coordinates pos
    inv_w: f32, // for perspective correct interpolation
    ndc_z: f32, // depth
}

fn to_screen(clip: Vec4, w: f32, h: f32) -> ScreenVert {
    let inv_w = 1. / clip.w;

    // clip space to normalized coords
    let ndc_x = clip.x * inv_w;
    let ndc_y = clip.y * inv_w;
    let ndc_z = clip.z * inv_w;

    // normalized to pixel coords
    let screen_x = (ndc_x * 0.5 + 0.5) * w;
    let screen_y = (1. - (ndc_y * 0.5 + 0.5)) * h;

    ScreenVert {
        pos: Vec2::new(screen_x, screen_y),
        inv_w,
        ndc_z,
    }
}
