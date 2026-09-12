pub mod builtin;

use crate::pipeline::Uniforms;
use glam::{Mat4, Vec4};

#[derive(Debug, Clone)]
pub struct PerObjectUniforms {
    model_matrix: Mat4,
}

impl PerObjectUniforms {
    pub fn new(matrix: Mat4) -> Self {
        Self {
            model_matrix: matrix,
        }
    }
    
    pub fn model_matrix(&self) -> &Mat4 {
        &self.model_matrix
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