use crate::geometry::ray::Ray;
use crate::material::base::Material;
use crate::math::vector::{Color, Vec3};
use crate::scene::object::Hit;

pub struct Metal {
    albedo: Color,
    fuzz: f32, // Fuzziness of the reflection
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        // Reflect the incoming ray with 100% faithful direction (mirror) based on the normal
        let reflected = ray.direction.reflect(&hit.normal);
        let reflected = reflected.unit() + (self.fuzz * Vec3::random_unit());

        let scattered = Ray::new(hit.p, reflected);

        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }

        None
    }
}
