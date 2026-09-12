use glam::Vec3;

#[macro_export]
macro_rules! verts {
    ($($pos:expr, $col:expr);+ $(;)?) => {
        vec![
            $(
            Vertex::new($pos, $col)
            ),+
        ]
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub col: Vec3
}

impl Vertex {
    pub fn new(pos: Vec3, col: Vec3) -> Self {
        Self {
            pos, col
        }
    }
}