use std::f32::consts::PI;

use camera::*;
use color::*;
use frame::*;
use light::*;
use trace::*;
use cgmath::{
    InnerSpace,
    Point3,
    Vector3,
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
    pub background: Color,
}

pub struct RenderOptions {
    /// When a ray intersects with a solid, the intersection point may be slightly above or below
    /// the surface that the ray hit. If it's below the surface, we'll incorrectly think that it's
    /// in a shadow because no light rays can reach it. So, before doing shadow computations, we
    /// offset the hit location by `shadow_bias` units along the normal of the surface to ensure
    /// that it's above the surface.
    pub shadow_bias: f32,

    /// `max_ray_depth` is the maximum level of recursive depth to which `cast_ray` will be
    /// computed. In other words, it's number of a ray will be reflected or refracted before we
    /// stop computing it. If set to 0, no reflections of refractions will be computed.
    pub max_ray_depth: usize,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            shadow_bias: 1e-4,
            max_ray_depth: 0,
        }
    }
}

/// Draws the provided scene with the provided render options.
pub fn draw(scene: &RenderScene, options: &RenderOptions) -> Frame<Color> {
    let camera = scene.camera;
    let (width, height) = camera.image_resolution;
    let mut frame = Frame::new(
        width,
        height,
        scene.background,
    );
    for y in 0..height {
        for x in 0..width {
            let ray = camera.pixel_ray(x, y);
            match cast_ray(scene, options, &ray, 0) {
                Some(mut color) => {
                    // Not sure when this should happen.
                    color = color.clamped();
                    frame.set(x, y, color);
                },
                None => {},
            }
        }
    }
    frame
}

/// Casts `ray` into the scene and returns the final computed color.
pub fn cast_ray(
    scene: &RenderScene,
    options: &RenderOptions,
    ray: &Ray3<f32>,
    ray_depth: usize,
) -> Option<Color> {
    match compute_scene_hit(scene, options, ray) {
        Some(hit) => {
            draw_hit(scene, options, ray, &hit, ray_depth)
        },
        _ => None,
    }
}

/// Casts `ray` into the scene and returns a `SceneObjectHit` corresponding to the first hit
/// if there is one, or nothing if not.
fn compute_scene_hit(
    scene: &RenderScene,
    options: &RenderOptions,
    ray: &Ray3<f32>
) -> Option<SceneObjectHit> {
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
                            (Some(previous_hit), Some(previous_distance))
                        }
                    },
                    _ => (Some(hit), Some(distance))
                };
            },
            None => continue,
        }
    }
    match current.0 {
        Some(mut hit) => {
            let adjusted_hit_point = hit.solid.point + (hit.solid.normal * options.shadow_bias);
            hit.solid.point = adjusted_hit_point;
            Some(hit)
        },
        None => None,
    }
}

fn draw_hit(
    scene: &RenderScene,
    options: &RenderOptions,
    ray: &Ray3<f32>,
    hit: &SceneObjectHit,
    ray_depth: usize
) -> Option<Color> {
    let light_color: Color = scene.lighting.lights.iter()
        .map(|light| { compute_light(light, scene, options, ray, hit) })
        .sum();

    // TODO: make this not awful.
    let reflected_ray = compute_reflected_ray(ray, hit);
    if ray_depth < options.max_ray_depth {
        cast_ray(scene, options, &reflected_ray, ray_depth + 1)
    } else {
        Some(light_color + scene.lighting.ambient)
    }
}

fn compute_reflected_ray(ray: &Ray3<f32>, hit: &SceneObjectHit) -> Ray3<f32> {
    let par_component = hit.solid.normal * ray.direction.dot(hit.solid.normal);
    let perp_component = ray.direction - par_component;
    Ray3::new(hit.solid.point, perp_component - par_component)
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
            if !hit_visible(
                hit.solid.point,
                point_light.position,
                scene,
                options,
            ) {
                return Color::from_rgb(0.0, 0.0, 0.0);
            }
            let light_direction = point_light.position - hit.solid.point;
            let light_distance = light_direction.magnitude();
            let normalized_light_direction = light_direction / light_distance;
            let m = hit.material.shading.brdf(
                &-ray.direction,
                &normalized_light_direction,
                &light.intensity,
                &hit.solid.normal
            );
            m / (4.0 * PI * light_distance * light_distance)
        },
        _ => panic!("oh nooooooo"),
    }
}

/// Returns `true` if `hit` is visible in `scene` from `point`.
fn hit_visible(
    start: Point3<f32>,
    point: Point3<f32>,
    scene: &RenderScene,
    options: &RenderOptions,
) -> bool {
    let unnormalized_ray = point - start;
    let distance = unnormalized_ray.magnitude();
    let ray = unnormalized_ray / distance;
    for obj in &scene.objects {
        match obj.trace(&Ray3::new(start, ray)) {
            Some(intersection) => {
                let distance_to_object = (intersection.solid.point - start).magnitude();
                if distance_to_object < distance {
                    return false;
                } else {
                    continue;
                }
            },
            None => continue,
        }
    }
    true
}