use super::object::{Hit, Object};
use crate::geometry::ray::{Ray, T_MAX, T_MIN};
use crate::math::numbers::Interval;
use crate::scene::camera::Camera;

pub struct Scene {
    pub name: String,
    pub cameras: Vec<Camera>,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn get_camera(&self, name: &String) -> Option<&Camera> {
        self.cameras.iter().find(|c| c.name == *name)
    }

    // TODO: write tests
    pub fn trace(&self, ray: &Ray) -> (Option<&Box<dyn Object>>, Option<Hit>) {
        ray_debug!("Tracing ray {} through the scene", ray);

        let t_min = T_MIN;
        let mut t_max = T_MAX;
        let mut closest: (Option<&Box<dyn Object>>, Option<Hit>) = (None, None);

        for obj in self.objects.iter() {
            match obj.hit_by(ray, Interval::new(t_min, t_max)) {
                (true, Some(hit_params)) => {
                    ray_debug!("Object hit: {}", obj.describe());

                    if hit_params.t < t_max {
                        ray_debug!("This object is closest so far");

                        // Any subsequent hits need to be closer to the ray's origin than this one
                        // (we want to find the first thing the Ray hits, starting from its origin
                        // and looking towards its direction)
                        t_max = hit_params.t;
                        closest = (Some(obj), Some(hit_params));
                    }
                }
                _ => (),
            }
        }

        closest
    }
}
