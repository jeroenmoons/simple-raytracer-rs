use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera = Camera::new(String::from("main"), Point::origin(), 1., 4., 16. / 9.);

    let center_one = Point::new(0., 0., -1.);
    let sphere_one = Sphere::new(center_one, 0.5);

    let center_two = Point::new(0., -100.5, -1.);
    let sphere_two = Sphere::new(center_two, 100.);

    Scene {
        name: String::from("Small sphere above a huge one"),
        cameras: vec![camera],
        objects: vec![Box::new(sphere_one), Box::new(sphere_two)],
    }
}
