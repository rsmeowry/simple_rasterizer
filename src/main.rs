pub mod asset;
pub mod camera;
mod framebuffer;
pub mod pipeline;
pub mod tri;
pub mod util;
pub mod world;

use crate::asset::Texture;
use crate::pipeline::shader::builtin::{PhongFS, PhongVS};
use crate::util::RoundN;
use crate::world::{Transform, World};
use glam::{Quat, Vec3};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 640;

fn main() {
    let mut win = minifb::Window::new(
        "my rasterizer! :D",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap();

    let mut pipeline = pipeline::Pipeline::new(WIDTH, HEIGHT);

    let tex = Texture::load("./assets/pusheen_albedo.png");
    let shader = PhongFS {
        light_dir: Vec3::new(0.5, -1.0, 0.3).normalize(),
        light_col: Vec3::new(1.0, 1.0, 0.95),
        ambient: 0.1,
        diffuse_k: 0.7,
        specular_k: 0.3,
        shininess: 128.0,
        tex,
    };
    let pusheen = asset::load_obj("./assets/pusheen_hi_res.obj", PhongVS, shader);
    let mut tf = Transform::default();

    let mut world = World::new();
    let cam = world.camera_mut();
    cam.pos = Vec3::new(0., 1., -6.);
    cam.fov = 90f32.to_radians();
    // cam.dir = (Vec3::Z - Vec3::Y * 0.5).normalize();
    // tf.rot = Quat::from_euler(EulerRot::XYZ, 0., 0., 0.);
    // cam.pos = Vec3::new(0., 0., 3.);
    // cam.dir = look_at_quat(cam.pos, Vec3::ZERO, Vec3::Y);

    let mut y = 0f32;
    let mut scale = 1f32;
    while win.is_open() && !win.is_key_down(Key::Escape) {
        world.begin_frame();

        if win.is_key_down(Key::F) {
            win.set_title(&format!(
                "{} FPS | my rasterizer! :D",
                (1. / world.delta_time()).round_n(1)
            ));
        }

        if win.is_key_down(Key::Left) {
            y -= world.delta_time() * 190. * if win.is_key_down(Key::LeftShift) { 5. } else { 1. };
        } else if win.is_key_down(Key::Right) {
            y += world.delta_time() * 190. * if win.is_key_down(Key::LeftShift) { 5. } else { 1. };
        }

        if win.is_key_down(Key::Down) {
            scale -= world.delta_time();
        } else if win.is_key_down(Key::Up) {
            scale += world.delta_time();
        }

        tf.scl = Vec3::ONE * scale;
        tf.rot = Quat::from_rotation_y(y.to_radians());
        world.draw_object(&pusheen, tf);

        // rendering to png requires objects
        if win.is_key_pressed(Key::R, KeyRepeat::No) {
            world.render_to_png(&mut pipeline, "./out.JPEG");
        }

        world.render(&mut pipeline);

        win.update_with_buffer(
            pipeline.buffer().color(),
            pipeline.buffer().width(),
            pipeline.buffer().height(),
        )
        .unwrap();
    }
}
