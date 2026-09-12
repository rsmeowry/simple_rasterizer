pub mod tri;
mod util;
mod framebuffer;

use glam::{Vec2, Vec3};
use minifb::{Key, Scale, ScaleMode, WindowOptions};
use crate::framebuffer::Framebuffer;

fn main() {
    let mut win = minifb::Window::new("my rasterizer! :D", 800, 640, WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::FitScreen,
        scale_mode: ScaleMode::Center,
        topmost: false,
        transparency: false,
        none: false,
    }).unwrap();

    let mut fb = Framebuffer::new(800, 640);

    while win.is_open() && !win.is_key_down(Key::Escape) {
        win.update_with_buffer(fb.color(), 800, 640).unwrap();
    }
}
