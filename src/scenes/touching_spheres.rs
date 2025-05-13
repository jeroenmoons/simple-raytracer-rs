use crate::material::diffuse::Lambert;
use crate::math::constants::PI;
use crate::math::vector::Color;
use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let r = (PI / 4.).cos();

    let camera = Camera::new(
        String::from("main"),
        Point::origin(),
        Point::new(0., 0., -1.),
        90.,
        16. / 9.,
    );

    let sphere_blue = Sphere::new(
        Point::new(-r, 0., -1.),
        r,
        Box::from(Lambert::new(Color::new(0., 0., 1.))),
    );

    let sphere_red = Sphere::new(
        Point::new(r, 0., -1.),
        r,
        Box::from(Lambert::new(Color::new(1., 0., 0.))),
    );

    Scene {
        name: String::from("Two spheres touching"),
        cameras: vec![camera],
        objects: vec![Box::new(sphere_blue), Box::new(sphere_red)],
    }
}
