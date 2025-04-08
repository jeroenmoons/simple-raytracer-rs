use crate::{math::vector::Point, scene::object::Object};

use super::ray::Ray;

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
    fn hit_by(&self, ray: &Ray) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = -2. * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        return discriminant >= 0.;

        // vec3 oc = center - r.origin();
        // auto a = dot(r.direction(), r.direction());
        // auto b = -2.0 * dot(r.direction(), oc);
        // auto c = dot(oc, oc) - radius*radius;
        // auto discriminant = b*b - 4*a*c;
        // return (discriminant >= 0);
    }
}
