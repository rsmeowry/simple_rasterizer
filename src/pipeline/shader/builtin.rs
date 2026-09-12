use crate::asset::Texture;
use crate::pipeline::shader::{FragmentShader, PerObjectUniforms, Varyings, VertexShader};
use crate::pipeline::vertex::Vertex;
use crate::pipeline::{Color, Uniforms};
use glam::{Mat4, Vec2, Vec3, Vec4};

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
pub struct SimpleTexCoords;

#[derive(Debug, Clone)]
pub struct UnlitTextured(pub Texture);

impl VertexShader<Vertex, TexCoord> for SimpleTexCoords {
    fn process(&self, vertex: &Vertex, obj: &PerObjectUniforms,  uniforms: &Uniforms) -> (Vec4, TexCoord) {
        let mvp: Mat4 = uniforms.proj * uniforms.view * obj.model_matrix();
        (mvp.mul_vec4(Vec4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0)), TexCoord(vertex.uv))
    }
}

impl FragmentShader<TexCoord> for UnlitTextured {
    fn apply(&self, var: &TexCoord, _uniforms: &Uniforms) -> Vec4 {
        let tex = &self.0;
        let sampled = tex.sample_nearest(var.0);
        Vec4::new(sampled.x, sampled.y, sampled.z, 1.)
    }
}

#[derive(Debug, Clone)]
pub struct PhongVS;

impl VertexShader<Vertex, PhongVaryings> for PhongVS {
    fn process(&self, vertex: &Vertex, obj: &PerObjectUniforms, uniforms: &Uniforms) -> (Vec4, PhongVaryings) {
        let world_pos = obj.model_matrix.transform_point3(vertex.pos);
        let normal = vertex.normal;
        let uv = vertex.uv;

        let mvp = uniforms.proj * uniforms.view * obj.model_matrix();
        (mvp.mul_vec4(Vec4::new(vertex.pos.x, vertex.pos.y, vertex.pos.z, 1.0)), PhongVaryings { world_pos, normal, uv })
    }
}

#[derive(Debug, Clone)]
pub struct PhongFS {
    pub light_dir: Vec3,
    pub light_col: Vec3,
    pub ambient: f32,
    pub diffuse_k: f32,
    pub specular_k: f32,
    pub shininess: f32,
    pub tex: Texture,
}

#[derive(Debug, Copy, Clone)]
pub struct PhongVaryings {
    pub normal: Vec3,
    pub uv: Vec2,
    pub world_pos: Vec3
}

impl Varyings for PhongVaryings {
    fn interpolate(a: &Self, b: &Self, c: &Self, weights: (f32, f32, f32)) -> Self {
        let uv = weights.0 * a.uv + weights.1 * b.uv + weights.2 * c.uv;
        let normal = weights.0 * a.normal + weights.1 * b.normal + weights.2 * c.normal;
        let world_pos = weights.0 * a.world_pos + weights.1 * b.world_pos + weights.2 * c.world_pos;

        Self { normal, uv, world_pos }
    }
}

impl FragmentShader<PhongVaryings> for PhongFS {
    fn apply(&self, var: &PhongVaryings, uniforms: &Uniforms) -> Vec4 {
        let view_dir = (uniforms.camera_pos - var.world_pos).normalize();
        let n = var.normal.normalize();
        let reflect_dir = (-self.light_dir).reflect(n);

        let diffuse = n.dot(self.light_dir).max(0.0);
        let specular = view_dir.dot(reflect_dir).max(0.0).powf(self.shininess);

        let base_color = self.tex.sample_nearest(var.uv);

        let intensity = self.ambient + self.diffuse_k * diffuse;
        let col = base_color * self.light_col * intensity + self.specular_k * specular;
        col.extend(1.)
    }
}