use glam::{Mat4, Vec2, Vec4};
use crate::pipeline::{Color, Uniforms};
use crate::pipeline::shader::{FragmentShader, PerObjectUniforms, Varyings, VertexShader};
use crate::pipeline::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct TexCoord(Vec2);

impl Varyings for Color {
    fn interpolate(a: &Self, b: &Self, c: &Self, weights: (f32, f32, f32)) -> Self {
        Self(weights.0 * a.0 + weights.1 * b.0 + weights.2 * c.0)
    }
}

impl Varyings for Vec2 {
    fn interpolate(a: &Self, b: &Self, c: &Self, weights: (f32, f32, f32)) -> Self {
        weights.0 * a + weights.1 * b + weights.2 * c
    }
}

impl Varyings for TexCoord {
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

#[derive(Debug, Copy, Clone)]
pub struct VisualizeUv;

impl VertexShader<Vertex, Vec2> for VisualizeUv {
    fn process(&self, vertex: &Vertex, obj: &PerObjectUniforms,  uniforms: &Uniforms) -> (Vec4, Vec2) {
        let mvp: Mat4 = uniforms.proj * uniforms.view * obj.model_matrix();
        (mvp.mul_vec4(Vec4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0)), vertex.uv)
    }
}

impl FragmentShader<Vec2> for VisualizeUv {
    fn apply(&self, var: &Vec2, _uniforms: &Uniforms) -> Vec4 {
        return Vec4::new(var.x, var.y, 1., 1.);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnlitTextured;

impl VertexShader<Vertex, TexCoord> for UnlitTextured {
    fn process(&self, vertex: &Vertex, obj: &PerObjectUniforms,  uniforms: &Uniforms) -> (Vec4, TexCoord) {
        let mvp: Mat4 = uniforms.proj * uniforms.view * obj.model_matrix();
        (mvp.mul_vec4(Vec4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0)), TexCoord(vertex.uv))
    }
}

impl FragmentShader<TexCoord> for UnlitTextured {
    fn apply(&self, var: &TexCoord, uniforms: &Uniforms) -> Vec4 {
        let tex = uniforms.tex();
        let Some(tex) = tex else { return Vec4::ZERO; };
        let sampled = tex.sample_nearest(var.0);
        Vec4::new(sampled.x, sampled.y, sampled.z, 1.)
    }
}
