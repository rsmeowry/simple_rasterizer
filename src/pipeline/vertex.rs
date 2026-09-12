use glam::{Vec2, Vec3};

#[macro_export]
macro_rules! verts {
    ($($pos:expr, $col:expr);+ $(;)?) => {
        vec![
            $(
            Vertex::new_col($pos, $col)
            ),+
        ]
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub col: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new_col(pos: Vec3, col: Vec3) -> Self {
        Self {
            pos,
            col,
            normal: Vec3::ZERO,
            uv: Vec2::ZERO,
        }
    }

    pub fn new_normal_uv(pos: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            pos,
            col: Vec3::ZERO,
            normal,
            uv,
        }
    }
}
