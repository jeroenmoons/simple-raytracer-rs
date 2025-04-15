use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let center_one = Point::new(0., 0., -8.);
    let sphere_one = Sphere::new(center_one, 5.);

    let center_two = Point::new(0.5, 0., -2.);
    let sphere_two = Sphere::new(center_two, 1.);

    Scene {
        name: String::from("Small sphere in front of larger one"),
        objects: vec![Box::new(sphere_one), Box::new(sphere_two)],
    }
}
