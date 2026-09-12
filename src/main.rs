pub mod tri;
mod util;
mod framebuffer;
pub mod pipeline;
pub mod camera;
pub mod world;
pub mod asset;

use crate::asset::Texture;
use crate::pipeline::object::AnyRenderObject;
use crate::pipeline::shader::builtin::UnlitTextured;
use crate::util::RoundN;
use crate::world::{Transform, World};
use glam::{FloatExt, Quat, Vec3};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, WindowOptions};
use std::ops::DerefMut;

const WIDTH: usize = 800;
const HEIGHT: usize = 640;

fn main() {
    let mut win = minifb::Window::new("my rasterizer! :D", WIDTH, HEIGHT, WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X1,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: false,
        transparency: false,
        none: false,
    }).unwrap();

    let mut pipeline = pipeline::Pipeline::new(WIDTH, HEIGHT);

    let pusheen = asset::load_obj("./assets/pusheen.obj", UnlitTextured, UnlitTextured);
    let tex = Texture::load("./assets/pusheen_albedo.png");
    let mut tf = Transform::default();

    let mut world = World::new();
    world.set_main_tex(tex);
    let cam = world.camera_mut();
    cam.pos = Vec3::new(0., 1., -6.);
    cam.fov = 90f32.to_radians();
    // cam.dir = (Vec3::Z - Vec3::Y * 0.5).normalize();
    // tf.rot = Quat::from_euler(EulerRot::XYZ, 0., 0., 0.);
    // cam.pos = Vec3::new(0., 0., 3.);
    // cam.dir = look_at_quat(cam.pos, Vec3::ZERO, Vec3::Y);

    let mut y = 0f32;
    while win.is_open() && !win.is_key_down(Key::Escape) {
        world.begin_frame();

        if win.is_key_down(Key::F) {
            win.set_title(&format!("{} FPS | my rasterizer! :D", (1. / world.delta_time()).round_n(1)));
        }

        if win.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            y -= world.delta_time() * 120.;
        } else if win.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            y += world.delta_time() * 120.;
        }

        tf.rot = Quat::from_rotation_y(y.to_radians());
        world.draw_object(&pusheen, tf);

        world.render(&mut pipeline);

        win.update_with_buffer(pipeline.buffer().color(), pipeline.buffer().width(), pipeline.buffer().height()).unwrap();
    }
}
