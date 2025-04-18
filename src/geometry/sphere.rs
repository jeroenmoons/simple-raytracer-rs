use super::ray::Ray;
use crate::math::numbers::Interval;
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
    // TODO: write tests
    fn hit_by(&self, ray: &Ray, within: Interval) -> (bool, Option<Hit>) {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return (false, None); // No solution
        }

        let discriminant_squared = discriminant.sqrt();
        let t = (h - discriminant_squared) / a; // Calculate t where ray intersects sphere

        // Find solution closest to the camera within acceptable range between camera and horizon
        if !within.contains(t) {
            let t = (h + discriminant_squared) / a;
            if !within.contains(t) {
                return (false, None);
            }
        }

        // Proper hit, calculate hit point and the outward normal (pointing from the center outward) at that point
        let p = ray.at(t);
        let outward_normal = (p - self.center).unit(); // We make the normal unit length, has a big impact on subsequent calculations!

        (true, Some(Hit::new(&ray, p, outward_normal, t)))
    }
}
