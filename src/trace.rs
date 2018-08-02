use cgmath::{
    BaseFloat,
    InnerSpace,
    Point3,
    Vector3,
};
use collision::{
    Continuous,
    Ray3,
    Sphere,
};
use color::*;
use material::*;
use collision;

/// A `SolidHit` object describes the nature of the intersection between a `Ray` and
/// a particular `Solid` object, including the point of intersection and the
/// normal of the solid's surface at that point.
///
/// All points and vectors are in world space.
#[derive(Copy, Debug, Clone)]
pub struct SolidHit {
    /// The point of intersection.
    pub point: Point3<f32>,

    /// The normal of the object's surface at the point of intersection.
    pub normal: Vector3<f32>,
}

pub trait Solid {
    /// If `ray` intersects this object, `trace` returns a `SolidHit` object detailing this
    /// intersection; otherwise it returns `None`.
    fn trace(&self, ray: &Ray3<f32>) -> Option<SolidHit>;
}

impl Solid for Sphere<f32> {
    fn trace(&self, ray: &Ray3<f32>) -> Option<SolidHit> {
        match Sphere::intersection(self, ray) {
            Some(intersection) => {
                let unnormalized_normal = intersection - self.center;
                let normal = unnormalized_normal / unnormalized_normal.magnitude();
                Some(SolidHit {
                    point: intersection,
                    normal,
                })
            },
            None => None
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub struct SceneObjectHit {
    pub solid: SolidHit,
    pub material: Material,
}

pub trait SceneObject {
    /// If `ray` intersects this object, `trace` returns a `SceneObjectHit` object detailing this
    /// intersection; otherwise it returns `None`.
    fn trace(&self, ray: &Ray3<f32>) -> Option<SceneObjectHit>;
}

pub struct SimpleObject {
    pub solid: Box<Solid>,
    pub material: Material,
}

impl SceneObject for SimpleObject {
    fn trace(&self, ray: &Ray3<f32>) -> Option<SceneObjectHit> {
        self.solid.trace(ray).map(|hit| {
            SceneObjectHit {
                solid: hit,
                material: self.material,
            }
        })
    }
}
