use crate::geometry::ray::Ray;
use crate::material::base::Material;
use crate::math::chance::random_f32;
use crate::math::vector::Color;
use crate::scene::object::Hit;

// Dielectric materials (like glass and water) both reflect and refract incoming rays.
// The material turns and incoming ray into both a reflected ray and a refracted ray, but for
// implementation in a renderer it is easier (because we are calculating many samples for a single
// pixel) to either reflect or refract for any one sample; the combined samples come down to the
// same aggregated result of both reflected and refracted rays.
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over the
    // refractive index of the enclosing medium.
    //
    // Index influencing how much light refracts when interacting with a material - these are
    // experimentally determined and for common materials quickly found on the internet.
    // (Nobody succeeded yet in calculating refractive indices for a material from first principles)
    pub refractive_index: f32,
    pub attenuation: Color,
}

impl Dielectric {
    pub fn new(refractive_index: f32, attenuation: Color) -> Self {
        Self {
            refractive_index,
            attenuation,
        }
    }

    pub fn new_glass() -> Self {
        Self::new(
            1.5,
            Color::new(1.0, 1.0, 1.0), // Glass absorbs nothing
        )
    }

    // Schlick's estimation of whether a ray will reflect or refract when interacting with glass
    pub fn reflectance(&self, cos_theta: f32, refractive_index: f32) -> f32 {
        let r0 = (1. - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        ray_debug!("DIELECTRIC - Scattering ray: {:?}", ray);

        let ri;
        let normal = hit.normal;

        if hit.front_face {
            ray_debug!("DIELECTRIC - hit front face");

            ri = 1.0 / self.refractive_index;
        } else {
            ray_debug!("DIELECTRIC - hit back face");

            ri = self.refractive_index;
        }

        let unit_direction = ray.direction.unit();

        let cos_theta = (-unit_direction).dot(normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction;
        if cannot_refract || (self.reflectance(cos_theta, ri) > random_f32(0., 1.)) {
            ray_debug!("DIELECTRIC - reflecting");

            direction = unit_direction.reflect(&normal);
        } else {
            ray_debug!("DIELECTRIC - refracting");

            direction = unit_direction.refract(&normal, ri);
        }

        Some((Ray::new(hit.p, direction), self.attenuation))
    }

    fn describe(&self) -> String {
        format!(
            "Dielectric material with refractive index {} and attenuation {}",
            self.refractive_index, self.attenuation,
        )
    }
}
