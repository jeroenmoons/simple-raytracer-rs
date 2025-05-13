use crate::material::diffuse::Lambert;
use crate::math::vector::Color;
use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera = Camera::new(
        String::from("main"),
        Point::origin(),
        Point::new(0., 0., -1.),
        90.,
        16. / 9.,
    );

    let sphere = Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        Box::from(Lambert::new(Color::new(0.5, 0.5, 0.5))),
    );

    Scene {
        name: String::from("Single lambert sphere"),
        cameras: vec![camera],
        objects: vec![Box::new(sphere)],
    }
}
