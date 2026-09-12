pub mod builtin;

use glam::{Mat4, Quat, Vec3, Vec4};
use crate::pipeline::Uniforms;

#[derive(Debug, Clone)]
pub struct PerObjectUniforms {
    model_matrix: Mat4,
    pos: Vec3,
    rot: Quat,
    scale: Vec3,
}

impl PerObjectUniforms {
    // TODO: lazy computation of matrix maybe idk
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.model_matrix = Mat4::from_scale_rotation_translation(self.scale, self.rot, self.pos);
    }

    pub fn set_rot(&mut self, rot: Quat) {
        self.rot = rot;
        self.model_matrix = Mat4::from_scale_rotation_translation(self.scale, self.rot, self.pos);
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.model_matrix = Mat4::from_scale_rotation_translation(self.scale, self.rot, self.pos);
    }

    pub fn model_matrix(&self) -> &Mat4 {
        &self.model_matrix
    }
}

impl Default for PerObjectUniforms {
    fn default() -> Self {
        Self {
            model_matrix: Mat4::IDENTITY,
            pos: Vec3::ZERO,
            rot: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

pub trait Varyings: Clone {
    fn interpolate(a: &Self, b: &Self, c: &Self, weights: (f32, f32, f32)) -> Self;
}

impl Varyings for () {
    fn interpolate(_a: &Self, _b: &Self, _c: &Self, _weights: (f32, f32, f32)) -> Self { }
}

pub trait VertexShader<In, V: Varyings> {
    fn process(&self, vertex: &In, obj: &PerObjectUniforms, uniforms: &Uniforms) -> (Vec4, V);
}

pub trait FragmentShader<V: Varyings> {
    fn apply(&self, var: &V, uniforms: &Uniforms) -> Vec4;
}