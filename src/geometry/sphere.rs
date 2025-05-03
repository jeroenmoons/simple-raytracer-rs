use super::ray::Ray;
use crate::material::base::Material;
use crate::math::numbers::Interval;
use crate::scene::object::Hit;
use crate::{math::vector::Point, scene::object::Object};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

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

        let discriminant_sqrt = discriminant.sqrt();
        let mut t = (h - discriminant_sqrt) / a; // Calculate t where ray intersects sphere

        // Find solution closest to the camera within acceptable range between camera and horizon
        if !within.surrounds(t) {
            t = (h + discriminant_sqrt) / a;
            if !within.surrounds(t) {
                return (false, None);
            }
        }

        ray_debug!("Ray hit sphere at t {}", t);

        // Proper hit, calculate hit point and the outward normal (pointing from the center outward) at that point
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius; // We make the normal unit length, has a big impact on subsequent calculations!

        (true, Some(Hit::new(&ray, p, outward_normal, t)))
    }

    fn describe(&self) -> String {
        format!(
            "Sphere at {} with radius {}, material: {} }}",
            self.center,
            self.radius,
            self.material.describe()
        )
    }
}
