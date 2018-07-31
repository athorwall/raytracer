extern crate sdl2;
extern crate raytracer;
extern crate collision;
extern crate cgmath;

use raytracer::sdl::*;
use raytracer::draw::*;
use raytracer::camera::*;
use collision::{
    Sphere,
};
use cgmath::{
    Matrix4,
    Point3,
    Vector3,
};
use sdl2::{
    event::Event,
};

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut canvas = create_sdl_canvas(&ctx, 1200, 900);

    let camera = {
        let mut mutable_camera = Camera::default();
        mutable_camera.eye = Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 5.0 });
        mutable_camera.image_resolution = (1200, 900);
        mutable_camera
    };

    let scene = RenderScene {
        objects: vec![
            Box::from(Sphere {
                center: Point3 { x: 0.0, y: 0.0, z: 0.0 },
                radius: 1.0,
            }),
        ],
        camera,
    };

    let frame = draw(&scene);
    render_to_canvas(&mut canvas, &frame);

    'main: loop {

        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                _               => continue
            }
        }

    }
}