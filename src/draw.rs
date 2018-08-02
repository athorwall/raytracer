use camera::*;
use color::*;
use frame::*;
use light::*;
use trace::*;
use cgmath::{
    InnerSpace,
};
use collision::{
    Ray3,
};

/// A `RenderScene` is a scene that requires no additional processing (e.g. transformations)
/// prior to being rendered.
pub struct RenderScene {
    pub objects: Vec<Box<Solid>>,
    pub camera: Camera,
    pub lighting: Lighting,
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
    let mut current: (Option<SolidHit>, Option<f32>) = (None, None);
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
            draw_hit(scene, ray, &hit)
        },
        _ => None,
    }
}

fn draw_hit(scene: &RenderScene, ray: &Ray3<f32>, hit: &SolidHit) -> Option<Color> {
    let light_color = scene.lighting.lights.iter()
        .map(|light| { compute_light(scene, ray, hit) })
        .sum();
    Some(light_color)
}

fn compute_light(scene: &RenderScene, ray: &Ray3<f32>, hit: &SolidHit) -> Color {
    Color::from_rgb(1.0, 1.0, 1.0)
}