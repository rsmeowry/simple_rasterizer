pub mod tri;
mod util;
mod framebuffer;
pub mod pipeline;
pub mod camera;
mod world;

use std::ops::DerefMut;
use glam::{FloatExt, Vec2, Vec3};
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

    let object: Box<dyn AnyRenderObject> = Box::new(RenderObject::new(verts![
        Vec3::new(-1.0, 0., 0.), Vec3::new(1., 0., 0.);
        Vec3::new(1.0, 0., 0.), Vec3::new(0., 1., 0.);
        Vec3::new(0., 1.5, 0.), Vec3::new(0., 0., 1.);
    ], vec![0, 1, 2], SimpleVertexColor, SimpleVertexColor));
    let mut object_tf = Transform::default();
    object_tf.set_pos(Vec3::new(0., 0., -3.));
    let mut world = World::new();

    while win.is_open() && !win.is_key_down(Key::Escape) {
        world.begin_frame();

        let y = world.time().sin().remap(-1., 1., -2., 0.);
        object_tf.set_pos(Vec3::new(0., y, -3.));
        world.draw_object(&object, object_tf);

        world.render(&mut pipeline);

        win.update_with_buffer(pipeline.buffer().color(), pipeline.buffer().width(), pipeline.buffer().height()).unwrap();
    }
}
