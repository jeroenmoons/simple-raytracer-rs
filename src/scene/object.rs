use crate::geometry::ray::Ray;
use crate::math::vector::{Point, Vec3};

// Parent for anything that is part of a scene and can have an effect on the rendered output
pub trait Object {
    fn hit_by(&self, ray: &Ray) -> (bool, Option<Hit>);
}

// Represents a Ray hitting an object
pub struct Hit {
    pub p: Point,     // Point at which Object is hit
    pub normal: Vec3, // The normal to the Object at the hit point
    pub t: f32,       // The Ray parameter at which it reaches the object
}

impl Hit {
    pub(crate) fn new(p: Point, normal: Vec3, t: f32) -> Self {
        Self { p, normal, t }
    }
}
