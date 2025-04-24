use crate::material::diffuse::Lambert;
use crate::math::vector::Color;
use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera = Camera::new(String::from("main"), Point::origin(), 1., 4., 16. / 9.);

    let material_one = Box::from(Lambert::new(Color::new(0.5, 0.5, 0.5)));
    let center_one = Point::new(0., 0., -8.);
    let sphere_one = Sphere::new(center_one, 5., material_one);

    let material_two = Box::from(Lambert::new(Color::new(0.5, 0.5, 0.5)));
    let center_two = Point::new(0.5, 0., -2.);
    let sphere_two = Sphere::new(center_two, 1., material_two);

    Scene {
        name: String::from("Small sphere in front of larger one"),
        cameras: vec![camera],
        objects: vec![Box::new(sphere_one), Box::new(sphere_two)],
    }
}
