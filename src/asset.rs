use crate::pipeline::object::{AnyRenderObject, RenderObject};
use crate::pipeline::shader::{FragmentShader, Varyings, VertexShader};
use crate::pipeline::vertex::Vertex;
use glam::{Vec2, Vec3};
use std::fmt::Debug;
use std::path::Path;
use tobj::LoadOptions;

pub fn load_obj<P: AsRef<Path> + Debug, V, VS, FS>(
    path: P,
    vs: VS,
    fs: FS,
) -> Box<dyn AnyRenderObject>
where
    V: Varyings + 'static,
    VS: VertexShader<Vertex, V> + 'static,
    FS: FragmentShader<V> + 'static,
{
    let (models, _mats) = tobj::load_obj(
        path,
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("failed to load obj file");

    let mesh = &models.first().unwrap().mesh;

    let vertex_cnt = mesh.positions.len() / 3;
    let mut vertices = Vec::with_capacity(vertex_cnt);

    for i in 0..vertex_cnt {
        let pos = Vec3::new(
            mesh.positions[i * 3],
            mesh.positions[i * 3 + 1],
            mesh.positions[i * 3 + 2],
        );

        // pos = Vec3::new(pos.x, pos.z, -pos.y);

        let normal = if !mesh.normals.is_empty() {
            -Vec3::new(
                mesh.normals[i * 3],
                mesh.normals[i * 3 + 1],
                mesh.normals[i * 3 + 2],
            )
        } else {
            Vec3::ZERO
        };

        let uv = if !mesh.texcoords.is_empty() {
            Vec2::new(mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1])
        } else {
            Vec2::ZERO
        };

        vertices.push(Vertex::new_normal_uv(pos, normal, uv))
    }

    Box::new(RenderObject::new(vertices, mesh.indices.clone(), vs, fs))
}

#[derive(Debug, Clone)]
pub struct Texture {
    width: u32,
    height: u32,
    pixels: Vec<Vec3>,
}

impl Texture {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let img = image::open(path).expect("failed to load texture").to_rgb8();

        let (width, height) = img.dimensions();
        let pixels = img
            .pixels()
            .map(|p| {
                Vec3::new(
                    p[0] as f32 / 256.0,
                    p[1] as f32 / 256.0,
                    p[2] as f32 / 256.0,
                )
            })
            .collect();

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn sample_nearest(&self, uv: Vec2) -> Vec3 {
        let u = uv.x.fract().rem_euclid(1.0);
        let v = uv.y.fract().rem_euclid(1.0);

        let x = (u * self.width as f32) as u32;
        let y = (v * self.height as f32) as u32;
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        self.pixels[(y * self.width + x) as usize]
    }
}
