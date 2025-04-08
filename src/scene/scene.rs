use crate::geometry::ray::Ray;

use super::object::Object;

pub struct Scene {
    pub name: String,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn hit_by(&self, ray: &Ray) -> bool {
        self.objects.iter().any(|o| o.hit_by(ray))
    }
}
