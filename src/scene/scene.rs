use super::object::{Hit, Object};
use crate::geometry::ray::{Ray, T_MAX, T_MIN};
use crate::math::vector::Color;

pub struct Scene {
    pub name: String,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Color> {
        let t_min = T_MIN;
        let mut t_max = T_MAX;
        let mut closest: (Option<&Box<dyn Object>>, Option<Hit>) = (None, None);

        for obj in self.objects.iter() {
            match obj.hit_by(ray, t_min, t_max) {
                (true, Some(hit_params)) => {
                    if hit_params.t < t_max {
                        t_max = hit_params.t; // Any subsequent hits need to be closer to the camera than this one
                        closest = (Some(obj), Some(hit_params));
                    }
                }
                _ => (),
            }
        }

        match closest {
            (Some(_obj), Some(hit)) => Some(
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
