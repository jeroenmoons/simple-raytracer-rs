use super::ray::Ray;
use crate::scene::object::Hit;
use crate::{math::vector::Point, scene::object::Object};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Object for Sphere {
    fn hit_by(&self, ray: &Ray) -> (bool, Option<Hit>) {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return (false, None); // No solution
        }

        let t = (h - discriminant.sqrt()) / a; // Calculate t where ray intersects sphere

        if t <= 0. {
            // Consider only positive solutions, in front of the camera. sSince the Ray originates
            // there any t < 0 are behind the camera and this invisible.
            return (false, None);
        }

        // Proper hit, calculate hit point and normal
        let p = ray.at(t);
        let normal = (p - self.center).unit();

        (true, Some(Hit::new(p, normal, t)))
    }
}
