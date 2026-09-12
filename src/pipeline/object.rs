use std::any::Any;
use std::marker::PhantomData;
use glam::Vec4;
use crate::pipeline::shader::{FragmentShader, PerObjectUniforms, Varyings, VertexShader};
use crate::pipeline::Uniforms;
use crate::pipeline::vertex::Vertex;

#[derive(Clone)]
pub struct RenderObject<V, VS, FS> where V: Varyings, VS: VertexShader<Vertex, V>, FS: FragmentShader<V> {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vs: VS,
    pub fs: FS,
    pub per_object: PerObjectUniforms,
    pd: PhantomData<V>
}

impl<V, VS, FS> RenderObject<V, VS, FS> where V: Varyings, VS: VertexShader<Vertex, V>, FS: FragmentShader<V> {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, vs: VS, fs: FS) -> Self {
        Self {
            vertices,
            indices,
            vs,
            fs,
            per_object: Default::default(),
            pd: Default::default(),
        }
    }
}

pub trait AnyRenderObject {
    fn indices(&self) -> &[u32];
    fn vertices(&self) -> &Vec<Vertex>;
    fn per_object_uniforms(&self) -> &PerObjectUniforms;
    fn do_vertex_stage(&self, vertex: &Vertex, uniforms: &Uniforms) -> (Vec4, Box<dyn Any>);
    fn do_frag_stage(&self, var: Box<dyn Any>, uniforms: &Uniforms) -> Vec4;
    fn interpolate_varyings(
        &self,
        a: &dyn Any, b: &dyn Any, c: &dyn Any,
        weights: (f32, f32, f32),
    ) -> Box<dyn Any>;
}

impl<V, VS, FS> AnyRenderObject for RenderObject<V, VS, FS> where V: Varyings + 'static, VS: VertexShader<Vertex, V>, FS: FragmentShader<V> {
    fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn per_object_uniforms(&self) -> &PerObjectUniforms {
        &self.per_object
    }

    fn do_vertex_stage(&self, vertex: &Vertex, uniforms: &Uniforms) -> (Vec4, Box<dyn Any>) {
        let (clip_pos, var) = self.vs.process(&vertex, &self.per_object, uniforms);
        (clip_pos, Box::new(var))
    }

    fn do_frag_stage(&self, var: Box<dyn Any>, uniforms: &Uniforms) -> Vec4 {
        self.fs
            .apply(var.downcast_ref::<V>().expect("invalid varyings type at frag stage!"), uniforms)
    }

    fn interpolate_varyings(&self, a: &dyn Any, b: &dyn Any, c: &dyn Any, weights: (f32,f32,f32)) -> Box<dyn Any> {
        let (a, b, c) = (
            a.downcast_ref::<V>().unwrap(),
            b.downcast_ref::<V>().unwrap(),
            c.downcast_ref::<V>().unwrap(),
        );
        Box::new(V::interpolate(a, b, c, weights))
    }
}
