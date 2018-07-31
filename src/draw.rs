use trace::*;
use camera::*;
use frame::*;
use color::*;
use cgmath::{
    InnerSpace,
};
use collision::{
    Ray3,
};

/// A `RenderScene` is a scene that requires no additional processing (e.g. transformations)
/// prior to being rendered.
pub struct RenderScene {
    pub objects: Vec<Box<Traceable>>,
    pub camera: Camera,
}

pub fn draw(scene: &RenderScene) -> Frame<Color> {
    let camera = scene.camera;
    let (width, height) = camera.image_resolution;
    let mut frame = Frame::new(
        width,
        height,
        Color::from_rgb(0.0, 0.0, 0.0)
    );
    for y in 0..height {
        for x in 0..width {
            let ray = camera.pixel_ray(x, y);
            match cast_ray(scene, &ray) {
                Some(color) => {
                    frame.set(x, y, color);
                },
                None => {},
            }
        }
    }
    frame
}

pub fn cast_ray(scene: &RenderScene, ray: &Ray3<f32>) -> Option<Color> {
    let mut current: (Option<TraceHit>, Option<f32>) = (None, None);
    for object in &scene.objects {
        match object.trace(&ray) {
            Some(hit) => {
                let distance = (hit.point - ray.origin).magnitude();
                current = match current {
                    (Some(previous_hit), Some(previous_distance)) => {
                        if distance < previous_distance {
                            (Some(hit), Some(distance))
                        } else {
                            current
                        }
                    },
                    _ => (Some(hit), Some(distance))
                };
            },
            None => continue,
        }
    }
    match current {
        (Some(hit), _) => {
            Some(hit.color)
        },
        _ => None,
    }
}