use cgmath::*;
use color::*;

pub struct Lighting {
    pub lights: Vec<Light>,
    pub ambient: Color,
}

pub struct Light {
    // TODO: make private
    pub light_type: LightType,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn point_light(pos: Vector3<f32>) -> Self {
        Light{
            light_type: LightType::Point(PointLight{
                position: pos,
            }),
            color: Color::from_rgb(1.0, 1.0, 1.0),
            intensity: 1.0,
        }
    }

    pub fn directional_light(dir: Vector3<f32>) -> Self {
        Light{
            light_type: LightType::Directional(DirectionalLight{
                direction: dir / dir.magnitude(),
            }),
            color: Color::from_rgb(1.0, 1.0, 1.0),
            intensity: 1.0,
        }
    }
}

pub enum LightType {
    Point(PointLight),
    Directional(DirectionalLight),
}

pub struct PointLight {
    pub position: Vector3<f32>,
}

pub struct DirectionalLight {
    pub direction: Vector3<f32>,
}