use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera = Camera::new(String::from("main"), Point::origin(), 1., 4., 16. / 9.);

    let center = Point::new(0., 0., -1.);
    let sphere = Sphere::new(center, 0.5);

    Scene {
        name: String::from("Single sphere"),
        cameras: vec![camera],
        objects: vec![Box::new(sphere)],
    }
}
