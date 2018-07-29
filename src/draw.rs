use cgmath::{
    BaseFloat,
    Point3,
    Vector3,
};
use collision::{
    Ray3,
};
use color::*;

struct TraceHit<S> where S: BaseFloat {
    point: Point3<S>,
    normal: Vector3<S>,

    // To be replaced by Material, or something like that, later
    color: Color,
}

trait Traceable {
    fn trace<S>(&self, ray: Ray3<S>) -> Option<TraceHit<S>> where S: BaseFloat;
}
