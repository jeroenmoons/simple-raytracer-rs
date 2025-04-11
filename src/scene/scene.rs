use super::object::Object;
use crate::geometry::ray::Ray;
use crate::math::vector::Color;

pub struct Scene {
    pub name: String,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Color> {
        // Find an object that is hit and use the normal at that hit spot to calculate the color
        let first_hit_object = self
            .objects
            .iter()
            .map(|o| (o, o.hit_by(ray))) // Calculates hit on every object in the scene...
            .filter(|(_o, (is_hit, _hit))| *is_hit) // Take only the objects that were actually hit
            .next(); // Use the first one to calculate a color

        match first_hit_object {
            Some((_o, (true, Some(hit)))) => Some(
                0.5 * Color::new(
                    hit.normal.x() + 1.,
                    hit.normal.y() + 1.,
                    hit.normal.z() + 1.,
                ),
            ),
            _ => None,
        }
    }
}
