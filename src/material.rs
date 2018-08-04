use std::f32::consts::PI;
use std::rc::Rc;

use color::*;
use cgmath::{
    Vector3,
    InnerSpace,
};

pub trait Shading {
    fn brdf(&self, ray: &Vector3<f32>, light: &Vector3<f32>, normal: &Vector3<f32>) -> f32;
}

#[derive(Copy, Clone)]
pub struct SimpleDiffuseShading {
    pub albedo: f32,
}

impl Shading for SimpleDiffuseShading {
    fn brdf(&self, ray: &Vector3<f32>, light: &Vector3<f32>, normal: &Vector3<f32>) -> f32 {
        let mut z = light.dot(*normal);
        if z < 0.0 {
            z = 0.0;
        }
        self.albedo / PI * z
    }
}

#[derive(Clone)]
pub struct Material {
    pub diffuse: Color,
    pub shading: Rc<Shading>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            diffuse: Color::from_rgb(1.0, 1.0, 1.0),
            shading: Rc::from(SimpleDiffuseShading {
                albedo: 0.18,
            }),
        }
    }
}
