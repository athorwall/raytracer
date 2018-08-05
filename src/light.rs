use cgmath::*;
use color::*;

pub struct Lighting {
    pub lights: Vec<Light>,
    pub ambient: Color,
}

pub struct Light {
    // TODO: make private
    pub light_type: LightType,
    pub intensity: Color,
}

impl Light {
    pub fn point_light(pos: Point3<f32>, intensity: Color) -> Self {
        Light{
            light_type: LightType::Point(PointLight{
                position: pos,
            }),
            intensity,
        }
    }

    pub fn directional_light(dir: Vector3<f32>) -> Self {
        Light{
            light_type: LightType::Directional(DirectionalLight{
                direction: dir / dir.magnitude(),
            }),
            intensity: Color::from_rgb(1.0, 1.0, 1.0),
        }
    }
}

pub enum LightType {
    Point(PointLight),
    Directional(DirectionalLight),
}

pub struct PointLight {
    pub position: Point3<f32>,
}

pub struct DirectionalLight {
    pub direction: Vector3<f32>,
}