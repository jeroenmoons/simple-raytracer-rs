use crate::math::vector::{Point, Vec3};

// Represents a ray originating at a Camera's eye point and propagating through the Scene
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_supports_travelling_along() {
        let cases = [
            ((1., 2., 3.), (4., 5., 6.), 2., (9., 12., 15.)),
            ((1., 2., 3.), (1., 0., 0.), 0.2, (1.2, 2., 3.)),
            ((1., 2., 3.), (0., 1., 0.), 0.2, (1., 2.2, 3.)),
            ((1., 2., 3.), (0., 0., 1.), 0.2, (1., 2., 3.2)),
        ];

        for (p, d, t, expected) in cases.iter() {
            let origin = Point::new(p.0, p.1, p.2);
            let direction = Vec3::new(d.0, d.1, d.2);
            let ray = Ray::new(origin, direction);

            let point_b = ray.at(*t);

            assert_eq!(point_b.x(), expected.0);
            assert_eq!(point_b.y(), expected.1);
            assert_eq!(point_b.z(), expected.2);
        }
    }
}
