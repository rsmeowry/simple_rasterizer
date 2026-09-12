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

    pub fn color(&self) -> &[u32] {
        &self.color
    }

    pub fn clear_depth(&mut self) {
        self.depth.fill(f32::INFINITY);
    }
}