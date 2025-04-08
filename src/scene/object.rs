use crate::geometry::ray::Ray;

pub trait Object {
    fn hit_by(&self, ray: &Ray) -> bool;
}
