pub mod tri;
mod util;
mod framebuffer;
pub mod pipeline;
pub mod camera;
mod world;

use std::ops::DerefMut;
use glam::{FloatExt, Quat, Vec2, Vec3};
use glam::camera::lh::view::look_at_quat;
use glam::camera::rh::view::look_to_quat;
use minifb::{Key, Scale, ScaleMode, WindowOptions};
use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::pipeline::object::{AnyRenderObject, RenderObject};
use crate::pipeline::shader::builtin::SimpleVertexColor;
use crate::pipeline::vertex::Vertex;
use crate::world::{Transform, World};

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

    let mut pipeline = pipeline::Pipeline::new(800, 640);

    let cube: Box<dyn AnyRenderObject> = Box::new(RenderObject::new(verts![
        // pos                      col (mapped from position, just for visual debugging)
        Vec3::new(-1., -1., -1.), Vec3::new(0., 0., 0.);  // 0: back-bottom-left  -> black
        Vec3::new( 1., -1., -1.), Vec3::new(1., 0., 0.);  // 1: back-bottom-right -> red
        Vec3::new( 1.,  1., -1.), Vec3::new(1., 1., 0.);  // 2: back-top-right    -> yellow
        Vec3::new(-1.,  1., -1.), Vec3::new(0., 1., 0.);  // 3: back-top-left     -> green
        Vec3::new(-1., -1.,  1.), Vec3::new(0., 0., 1.);  // 4: front-bottom-left -> blue
        Vec3::new( 1., -1.,  1.), Vec3::new(1., 0., 1.);  // 5: front-bottom-right-> magenta
        Vec3::new( 1.,  1.,  1.), Vec3::new(1., 1., 1.);  // 6: front-top-right   -> white
        Vec3::new(-1.,  1.,  1.), Vec3::new(0., 1., 1.);  // 7: front-top-left    -> cyan
        ], vec![
        // front (z = +1)
        4, 5, 6,  4, 6, 7,
        // back (z = -1)
        1, 0, 3,  1, 3, 2,
        // left (x = -1)
        0, 4, 7,  0, 7, 3,
        // right (x = +1)
        5, 1, 2,  5, 2, 6,
        // top (y = +1)
        3, 7, 6,  3, 6, 2,
        // bottom (y = -1)
        0, 1, 5,  0, 5, 4,
    ], SimpleVertexColor, SimpleVertexColor));
    let mut cube_tf = Transform::default();
    cube_tf.pos = Vec3::new(0., 0., 0.);

    let tri: Box<dyn AnyRenderObject> = Box::new(RenderObject::new(verts![
        Vec3::new(-1.0, 0., 0.), Vec3::new(1., 0., 0.);
        Vec3::new(1.0, 0., 0.), Vec3::new(0., 1., 0.);
        Vec3::new(0., 1.5, 0.), Vec3::new(0., 0., 1.);
    ], vec![2, 1, 0], SimpleVertexColor, SimpleVertexColor));
    let mut tri_tf = Transform::default();
    tri_tf.pos = Vec3::new(0., 0., 0.);
    let mut world = World::new();
    let cam = world.camera_mut();
    cam.pos = Vec3::new(0., 3., -6.);
    cam.fov = 90f32.to_radians();
    cam.dir = (Vec3::Z - Vec3::Y * 0.5).normalize();
    // cam.pos = Vec3::new(0., 0., 3.);
    // cam.dir = look_at_quat(cam.pos, Vec3::ZERO, Vec3::Y);

    while win.is_open() && !win.is_key_down(Key::Escape) {
        world.begin_frame();

        let y = world.time().sin() * 3.;
        let rot = Quat::from_rotation_y(world.time());
        tri_tf.pos = Vec3::new(0., y, 0.);
        cube_tf.rot = rot;
        cube_tf.scl = Vec3::new(((world.time() + 3.).sin().abs() * 4.).max(0.2), 1., ((world.time() * 2.).cos().abs() * 5.).max(1.));

        world.draw_object(&cube, cube_tf);
        // world.draw_object(&tri, tri_tf);

        world.render(&mut pipeline);

        win.update_with_buffer(pipeline.buffer().color(), pipeline.buffer().width(), pipeline.buffer().height()).unwrap();
    }
}
