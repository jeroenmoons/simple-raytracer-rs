#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)] // guarantees layout compatibility as long as the struct has exactly one non-ZST (zero-sized type) field.
pub struct Vec3 {
    inner: glam::Vec3, // Use library under the hood, implements operations more efficiently
}

// These are type aliases, using these to make the code easier to read.
// For the compiler these are completely interchangeable though! When a Vec3 is expected, Color and
// Point can be passed without getting any errors.
pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: glam::Vec3::new(x, y, z),
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }

    pub fn y(&self) -> f32 {
        self.inner.y
    }

    pub fn z(&self) -> f32 {
        self.inner.z
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.inner.dot(other.inner)
    }

    // TODO: Implement operations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_represents_vectors() {
        let a_vector = Vec3::new(20., 5., 0.);

        assert_eq!(a_vector.x(), 20.0);
        assert_eq!(a_vector.y(), 5.);
        assert_eq!(a_vector.z(), 0.);
    }

    #[test]
    fn it_represents_colors_as_vectors() {
        let a_color = Color::new(1.0, 0.5, 0.3);

        assert_eq!(a_color.x(), 1.0);
        assert_eq!(a_color.y(), 0.5);
        assert_eq!(a_color.z(), 0.3);
    }

    #[test]
    fn it_represents_points_as_vectors() {
        let a_point = Point::new(0.5, 0.42, 0.8);

        assert_eq!(a_point.x(), 0.5);
        assert_eq!(a_point.y(), 0.42);
        assert_eq!(a_point.z(), 0.8);
    }

    #[test]
    fn it_calculates_dot_product_correctly() {
        let cases = [
            ((1., 2., 3.), (4., -5., 6.), 12.),    // basic
            ((1., 0., 0.), (0., 1., 0.), 0.),      // orthogonal vectors, dot product zero
            ((2., 3., 4.), (4., 6., 8.), 58.),     // parallel vectors
            ((2., 3., 4.), (0., 0., 0.), 0.),      // zero vector
            ((2., 3., 4.), (-1., -2., -3.), -20.), // negative vector
            ((1., 8., 3.), (1., 8., 3.), 74.),     // identical vectors
        ];

        for (a, b, expected) in cases.iter() {
            let vector_a = Vec3::new(a.0, a.1, a.2);
            let vector_b = Vec3::new(b.0, b.1, b.2);

            assert_eq!(vector_a.dot(vector_b), *expected);
        }
    }
}
