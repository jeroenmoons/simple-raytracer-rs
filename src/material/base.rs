use crate::geometry::ray::Ray;
use crate::math::vector::Color;
use crate::scene::object::Hit;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}
