use crate::material::diffuse::Diffuse;
use crate::math::vector::Color;
use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera = Camera::new(String::from("main"), Point::origin(), 1., 4., 16. / 9.);

    let sphere = Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        Box::from(Diffuse::new(Color::new(0.5, 0.5, 0.5))),
    );

    Scene {
        name: String::from("Single basic diffuse sphere"),
        cameras: vec![camera],
        objects: vec![Box::new(sphere)],
    }
}
