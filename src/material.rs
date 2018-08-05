use std::f32::consts::PI;
use std::rc::Rc;

use color::*;
use cgmath::{
    Vector3,
    InnerSpace,
};

pub trait Shading {
    fn brdf(
        &self,
        ray: &Vector3<f32>,
        light: &Vector3<f32>,
        intensity: &Color,
        normal:
        &Vector3<f32>
    ) -> Color;
}

#[derive(Copy, Clone)]
pub struct SimpleDiffuseShading {
    pub diffuse_color: Color,
    pub albedo: f32,
}

impl Shading for SimpleDiffuseShading {
    fn brdf(
        &self,
        ray: &Vector3<f32>,
        light: &Vector3<f32>,
        intensity: &Color,
        normal: &Vector3<f32>
    ) -> Color {
        let mut z = light.dot(*normal);
        if z < 0.0 {
            z = 0.0;
        }
        *intensity * self.diffuse_color * self.albedo / PI * z
    }
}

#[derive(Copy, Clone)]
pub struct PhongShading {
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub specular_exponent: i32,
}

impl Shading for PhongShading {
    fn brdf(
        &self,
        ray: &Vector3<f32>,
        light: &Vector3<f32>,
        intensity: &Color,
        normal: &Vector3<f32>
    ) -> Color {
        let mut z = light.dot(*normal);
        if z < 0.0 {
            z = 0.0;
        }
        let diffuse = self.diffuse_color / PI * z;

        let v = -*ray;
        let r = 2.0 * (normal.dot(*light)) * normal - light;
        let specular = self.specular_color * (v.dot(r).powi(self.specular_exponent));

        *intensity * (diffuse + specular)
    }
}

#[derive(Clone)]
pub struct Material {
    pub shading: Rc<Shading>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            shading: Rc::from(SimpleDiffuseShading {
                diffuse_color: Color::from_rgb(1.0, 1.0, 1.0),
                albedo: 0.18,
            }),
        }
    }
}
