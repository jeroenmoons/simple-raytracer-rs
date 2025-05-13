use crate::material::dielectric::Dielectric;
use crate::material::diffuse::Lambert;
use crate::material::metal::Metal;
use crate::math::vector::Color;
use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera_main = Camera::new(
        String::from("main"),
        Point::origin(),
        Point::new(0., 0., -1.),
        90.,
        16. / 9.,
    );

    let camera_vantage = Camera::new(
        String::from("vantage"),
        Point::new(-2., 2., 1.),
        Point::new(0., 0., -1.),
        75.,
        16. / 9.,
    );

    let ground_sphere = Sphere::new(
        Point::new(0., -100.5, -1.),
        100.,
        Box::from(Lambert::new(Color::new(0.8, 0.8, 0.))),
    );

    let center_sphere = Sphere::new(
        Point::new(0., 0., -1.2),
        0.5,
        Box::from(Lambert::new(Color::new(0.1, 0.2, 0.5))),
    );

    let left_sphere = Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        Box::from(Dielectric::new_glass()),
    );

    let left_sphere_inside = Sphere::new(
        Point::new(-1., 0., -1.),
        0.4,
        Box::from(Dielectric::new(0.66, Color::new(0.1, 1.0, 1.0))),
    );

    let right_sphere = Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        Box::from(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
    );

    Scene {
        name: String::from("Playground scene - contents may change at any time"),
        cameras: vec![camera_main, camera_vantage],
        objects: vec![
            Box::new(ground_sphere),
            Box::new(center_sphere),
            Box::new(left_sphere),
            Box::new(left_sphere_inside),
            Box::new(right_sphere),
        ],
    }
}
