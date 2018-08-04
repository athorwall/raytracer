extern crate sdl2;
extern crate raytracer;
extern crate collision;
extern crate cgmath;

use raytracer::{
    camera::*,
    color::*,
    draw::*,
    light::*,
    sdl::*,
    trace::*,
    material::*,
};
use collision::{
    Sphere,
    Plane,
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
    let camera = {
        let mut mutable_camera = Camera::default();
        mutable_camera.eye = Matrix4::from_translation(Vector3 { x: 0.0, y: 0.8, z: 5.0 });
        mutable_camera.image_resolution = (1200, 900);
        mutable_camera
    };

    let scene = RenderScene {
        objects: vec![
            Box::from(SimpleObject {
                solid: Box::from(Sphere {
                    center: Point3 { x: -1.8, y: 1.5, z: 0.0 },
                    radius: 1.5,
                }),
                material: Material::new(),
            }),
            Box::from(SimpleObject {
                solid: Box::from(Sphere {
                    center: Point3 { x: 1.5, y: 1.0, z: 1.0 },
                    radius: 1.0,
                }),
                material: Material::new(),
            }),
            Box::from(SimpleObject {
                solid: Box::from(Sphere {
                    center: Point3 { x: 0.2, y: 0.5, z: 2.0 },
                    radius: 0.5,
                }),
                material: Material::new(),
            }),
            Box::from(SimpleObject {
                solid: Box::from(Plane {
                    n: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
                    d: 0.0,
                }),
                material: Material::new(),
            }),
        ],
        camera,
        lighting: Lighting {
            ambient: Color::from_rgb(0.0, 0.0, 0.0),
            lights: vec![
                Light::point_light(Point3 { x: 3.0, y: 3.0, z: 4.0 }),
                Light::point_light(Point3 { x: -3.0, y: 1.0, z: 2.0 }),
            ],
        },
    };

    draw_and_wait(&scene);
}

fn draw_and_wait(scene: &RenderScene) {
    let ctx = sdl2::init().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut canvas = create_sdl_canvas(&ctx, 1200, 900);

    let frame = draw(&scene, &RenderOptions::default());
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