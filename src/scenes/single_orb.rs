use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let center = Point::new(0., 0., -1.);
    let sphere = Sphere::new(center, 0.5);

    Scene {
        name: String::from("Single sphere"),
        objects: vec![Box::new(sphere)],
    }
}
