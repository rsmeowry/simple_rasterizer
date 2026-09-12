use glam::Vec3;

pub struct Framebuffer {
    width: usize,
    height: usize,
    color: Vec<u32>,
    depth: Vec<f32>
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color: vec![0xF2_00_00; width * height],
            depth: vec![f32::INFINITY; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn color(&self) -> &[u32] {
        &self.color
    }

    pub fn depth_at(&self, x: usize, y: usize) -> f32 {
        self.depth[y * self.width + x]
    }

    pub fn set_col(&mut self, x: usize, y: usize, color: u32) {
        self.color[y * self.width + x] = color;
    }
    pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) {
        self.depth[y * self.width + x] = depth;
    }

    pub fn clear(&mut self, color: u32) {
        self.color.fill(color);
    }

    pub fn clear_col3(&mut self, color: Vec3) {
        self.clear(col3_to_u32(color))
    }

    pub fn clear_depth(&mut self) {
        self.depth.fill(f32::INFINITY);
    }
}

pub fn col3_to_u32(col: Vec3) -> u32 {
    let r = (col.x * 255f32).round() as u32;
    let g = (col.y * 255f32).round() as u32;
    let b = (col.z * 255f32).round() as u32;
    return r << 16 | g << 8 | b;
}