use crate::material::diffuse::Lambert;
use crate::material::metal::Metal;
use crate::math::vector::Color;
use crate::scene::camera::Camera;
use crate::{geometry::sphere::Sphere, math::vector::Point, scene::scene::Scene};

pub fn generate() -> Scene {
    let camera = Camera::new(String::from("main"), Point::origin(), 1., 4., 16. / 9.);

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
        Box::from(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3)),
    );

    let right_sphere = Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        Box::from(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
    );

    Scene {
        name: String::from("Two metal spheres next to a Lambert sphere on a Lambert ground"),
        cameras: vec![camera],
        objects: vec![
            Box::new(ground_sphere),
            Box::new(center_sphere),
            Box::new(left_sphere),
            Box::new(right_sphere),
        ],
    }
}
