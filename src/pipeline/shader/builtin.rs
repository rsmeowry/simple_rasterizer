use glam::{Mat4, Vec4};
use crate::pipeline::{Color, Uniforms};
use crate::pipeline::shader::{FragmentShader, PerObjectUniforms, Varyings, VertexShader};
use crate::pipeline::vertex::Vertex;

impl Varyings for Color {
    fn interpolate(a: &Self, b: &Self, c: &Self, weights: (f32, f32, f32)) -> Self {
        Self(weights.0 * a.0 + weights.1 * b.0 + weights.2 * c.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SimpleVertexColor;

impl VertexShader<Vertex, Color> for SimpleVertexColor {
    fn process(&self, vertex: &Vertex, obj: &PerObjectUniforms,  uniforms: &Uniforms) -> (Vec4, Color) {
        let mvp: Mat4 = uniforms.proj * uniforms.view * obj.model_matrix();
        (mvp.mul_vec4(Vec4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0)), Color(vertex.col))
    }
}

impl FragmentShader<Color> for SimpleVertexColor {
    fn apply(&self, var: &Color, _uniforms: &Uniforms) -> Vec4 {
        return Vec4::new(var.0.x, var.0.y, var.0.z, 1.);
    }
}