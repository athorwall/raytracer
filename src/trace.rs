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
use collision;

/// A `TraceHit` object describes the nature of the intersection between a `Ray` and
/// a particular `Traceable` object, including the point of intersection and the
/// normal and material properties of the object's surface at that point.
///
/// All points and vectors are in world space.
#[derive(Copy, Debug, Clone)]
pub struct TraceHit {
    /// The point of intersection.
    pub point: Point3<f32>,

    /// The normal of the object's surface at the point of intersection.
    pub normal: Vector3<f32>,

    /// The material properties of the object's surface at the point of intersection.
    // To be replaced by Material, or something like that, later.
    pub color: Color,
}

/// Types implement `Traceable` if they can be "traced", i.e. they can provide information
/// about whether a particular ray intersects with them, and the details of that intersection
/// if so.
pub trait Traceable {
    /// If `ray` intersects this object, `trace` returns a `TraceHit` object detailing this
    /// intersection; otherwise it returns `None`.
    fn trace(&self, ray: &Ray3<f32>) -> Option<TraceHit>;
}

impl Traceable for Sphere<f32> {
    fn trace(&self, ray: &Ray3<f32>) -> Option<TraceHit> {
        match Sphere::intersection(self, ray) {
            Some(intersection) => {
                let unnormalized_normal = intersection - self.center;
                let normal = unnormalized_normal / unnormalized_normal.magnitude();
                Some(TraceHit {
                    point: intersection,
                    normal,
                    // TODO: materials!
                    color: Color::from_rgb(1.0, 0.0, 0.0),
                })
            },
            None => None
        }
    }
}
