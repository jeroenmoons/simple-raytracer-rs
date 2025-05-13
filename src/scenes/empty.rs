use crate::math::vector::Point;
use crate::scene::camera::Camera;
use crate::scene::scene::Scene;

pub fn generate() -> Scene {
    let camera = Camera::new(
        String::from("main"),
        Point::origin(),
        Point::new(0., 0., -1.),
        90.,
        16. / 9.,
    );

    Scene {
        name: String::from("Empty"),
        cameras: vec![camera],
        objects: vec![],
    }
}
