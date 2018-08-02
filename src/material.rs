use color::*;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            diffuse: Color::from_rgb(1.0, 1.0, 1.0),
            albedo: 0.18,
        }
    }
}