use std::fmt;
use std::fmt::Display;

use crate::math::vector::{Point, Vec3};

// T_MIN and T_MAX define the acceptable range within which a Ray intersection with an Object must lie.
// Setting T_MIN to 0 means we might find intersections VERY close to the ray's origin, and due to the limited
// precision of a f32 number, rounding errors can cause use to find solutions that are not perfectly flush with
// the surface of a body. If we end up just inside, a T_MIN of 0 means we will intersect with the object again.
// The net result is that rays in this situation will become very dark and cause 'shadow acne'. Setting T_MIN
// a bit higher than 0 makes the algorithm less sensitive to this effect.
pub const T_MIN: f32 = 0.001;

pub const T_MAX: f32 = 1000.0; // Arbitrary value for now, what is reasonable?

// Represents a ray originating at a Camera's eye point and propagating through the Scene
#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(from: Point, to: Point) -> Self {
        Self {
            origin: from,
            direction: to - from,
        }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ray(origin {}, direction {})",
            self.origin, self.direction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_supports_travelling_along() {
        let cases = [
            ((1., 1., 1.), (2., 2., 2.), 0.5, (1.5, 1.5, 1.5)),
            ((1., 1., 1.), (3., 1., 1.), 5., (11., 1., 1.)),
            ((1., 1., 1.), (1., 5., 1.), 5., (1., 21., 1.)),
            ((1., 1., 1.), (1., 1., 2.), 5., (1., 1., 6.)),
        ];

        for (from, to, t, expected) in cases.iter() {
            let from = Point::new(from.0, from.1, from.2);
            let to = Vec3::new(to.0, to.1, to.2);
            let ray = Ray::new(from, to);

            let point_b = ray.at(*t);

            assert_eq!(point_b.x(), expected.0);
            assert_eq!(point_b.y(), expected.1);
            assert_eq!(point_b.z(), expected.2);
        }
    }
}
