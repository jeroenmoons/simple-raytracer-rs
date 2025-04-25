use crate::geometry::ray::Ray;
use crate::material::base::Material;
use crate::math::vector::{Color, Vec3};
use crate::scene::object::Hit;

pub struct Diffuse {
    pub attenuation: Color,
}

// Basic diffuse material:
// Scatters rays into a random direction within the hemisphere of the normal at the hit point.
impl Diffuse {
    pub fn new(attenuation: Color) -> Self {
        Self { attenuation }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let random_scatter = Vec3::random_unit_on_hemisphere(&hit.normal);

        Some((Ray::new(hit.p, random_scatter), self.attenuation))
    }
}

// Basic Lambert material:
// Scatters rays in a random direction but favours directions close to the normal
pub struct Lambert {
    pub albedo: Color,
}

impl Lambert {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambert {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        // To make the material Lambertian, the random scatter should be more likely to stick to the normal,
        // this can be done by ADDING a random vector to the normal instead of flat-out replacing it
        let mut lambert_scatter = hit.normal + Vec3::random_unit();

        // Using the scatter method below gives heavier shadows, not entirely sure why exactly
        // let mut lambert_scatter = hit.normal + Vec3::random_unit_on_hemisphere(&hit.normal);

        if lambert_scatter.near_zero() {
            // If the scatter vector is almost zero it can lead to problems (NaN, infinities),
            // if we come too close to zero we use the normal instead to avoid these issues.
            lambert_scatter = hit.normal;
        }

        Some((Ray::new(hit.p, lambert_scatter), self.albedo))
    }
}
