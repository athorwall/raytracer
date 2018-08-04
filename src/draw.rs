use std::f32::consts::PI;

use camera::*;
use color::*;
use frame::*;
use light::*;
use trace::*;
use cgmath::{
    InnerSpace,
    Point3,
};
use collision::{
    Ray3,
};

/// A `RenderScene` is a scene that requires no additional processing (e.g. transformations)
/// prior to being rendered.
pub struct RenderScene {
    pub objects: Vec<Box<SceneObject>>,
    pub camera: Camera,
    pub lighting: Lighting,
}

pub struct RenderOptions {
    pub shadow_bias: f32,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            shadow_bias: 1e-4,
        }
    }
}

pub fn draw(scene: &RenderScene, options: &RenderOptions) -> Frame<Color> {
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
            match cast_ray(scene, options, &ray) {
                Some(color) => {
                    frame.set(x, y, color);
                },
                None => {},
            }
        }
    }
    frame
}

pub fn cast_ray(scene: &RenderScene, options: &RenderOptions, ray: &Ray3<f32>) -> Option<Color> {
    let mut current: (Option<SceneObjectHit>, Option<f32>) = (None, None);
    for object in &scene.objects {
        match object.trace(&ray) {
            Some(hit) => {
                let distance = (hit.solid.point - ray.origin).magnitude();
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
            draw_hit(scene, options, ray, &hit)
        },
        _ => None,
    }
}

fn draw_hit(scene: &RenderScene, options: &RenderOptions, ray: &Ray3<f32>, hit: &SceneObjectHit) -> Option<Color> {
    let light_color = scene.lighting.lights.iter()
        .map(|light| { compute_light(light, scene, options, ray, hit) })
        .sum();
    Some(light_color)
}

fn compute_light(
    light: &Light,
    scene: &RenderScene,
    options: &RenderOptions,
    ray: &Ray3<f32>,
    hit: &SceneObjectHit
) -> Color {
    match light.light_type {
        LightType::Point(ref point_light) => {
            if hit_visible(
                hit.solid,
                point_light.position,
                scene,
                options,
            ) {
                return Color::from_rgb(0.0, 0.0, 0.0);
            }
            let light_direction = point_light.position - hit.solid.point;
            let normalized_light_direction = light_direction / light_direction.magnitude();
            let angle = light_direction.dot(hit.solid.normal);
            let nonnegative_angle = if angle < 0.0 { 0.0 } else { angle };
            light.color * (hit.material.albedo / PI * nonnegative_angle)
        },
        _ => panic!("oh nooooooo"),
    }
}

/// Returns `true` if `hit` is visible in `scene` from `point`.
fn hit_visible(
    hit: SolidHit,
    point: Point3<f32>,
    scene: &RenderScene,
    options: &RenderOptions,
) -> bool {
    let adjusted_start = hit.point + (hit.normal * options.shadow_bias);
    let unnormalized_ray = point - adjusted_start;
    let distance = unnormalized_ray.magnitude();
    let ray = unnormalized_ray / distance;
    for obj in &scene.objects {
        match obj.trace(&Ray3::new(adjusted_start, ray)) {
            Some(intersection) => {
                let distance_to_object = (intersection.solid.point - adjusted_start).magnitude();
                if distance_to_object < distance {
                    return true;
                } else {
                    continue;
                }
            },
            None => continue,
        }
    }
    false
}