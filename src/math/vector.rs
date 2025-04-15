use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)] // guarantees layout compatibility as long as the struct has exactly one non-ZST (zero-sized type) field.
pub struct Vec3 {
    // This Vec3 struct serves as an abstraction layer between the rendering algorithms and the
    // linear algebra library that implements the math efficiently. At some point I may want to try
    // other libraries to compare the performance.
    inner: glam::Vec3, // Use library under the hood, implements operations more efficiently
}

// These are type aliases, using these to make the code easier to read.
// For the compiler these are completely interchangeable though! When a Vec3 is expected, Color and
// Point can be passed without getting any errors.
pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn origin() -> Self {
        Self::new(0., 0., 0.)
    }

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

    pub fn length_squared(&self) -> f32 {
        self.inner.length_squared()
    }

    // Got into a rabbit hole here because I did not understand why the argument b to .dot(b) is not
    // consumed when it is called. I could still use b after passing it into a call to .dot because
    // Vec3 implements the Copy trait, and so do its fields, making it stack data rather than heap
    // data. The rules for ownership and borrowing are not applied here.
    //
    // Relevant excerpt from the Rust book - https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // """
    // Rust has a special annotation called the Copy trait that we can place on types that are
    // stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type
    // implements the Copy trait, variables that use it do not move, but rather are trivially copied,
    // making them still valid after assignment to another variable.
    // """
    pub fn dot(&self, other: Self) -> f32 {
        self.inner.dot(other.inner)
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            inner: self.inner.cross(other.inner),
        }
    }

    pub fn unit(&self) -> Self {
        Self {
            inner: self.inner.normalize(),
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({:.2}, {:.2}, {:.2})", self.x(), self.y(), self.z())
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { inner: -self.inner }
    }
}

// Support inline addition Vec3 + Vec3
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            inner: self.inner + other.inner,
        }
    }
}

// Support inline subtrqction Vec3 + Vec3
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            inner: self.inner - other.inner,
        }
    }
}

// Support inline product Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            inner: self.inner * other.inner,
        }
    }
}

// Support inline product Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self {
            inner: self.inner * other,
        }
    }
}

// Support inline product f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            inner: self * other.inner,
        }
    }
}

// Support inline division Vec3 / Vec3
impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            inner: self.inner / other.inner,
        }
    }
}

// Support inline division Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Self {
            inner: self.inner / other,
        }
    }
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

    #[test]
    fn it_calculates_cross_product_correctly() {
        let cases = [
            ((1., 2., 3.), (4., -5., 6.), (27., 6., -13.)), // basic
            ((1., 0., 0.), (0., 1., 0.), (0., 0., 1.)),     // orthogonal vectors
            ((1., 2., 3.), (2., 4., 6.), (0., 0., 0.)),     // parallel vectors
            ((1., 2., 3.), (1., 2., 3.), (0., 0., 0.)),     // identical vectors
            ((1., 2., 3.), (0., 0., 0.), (0., 0., 0.)),     // zero vector
            ((-1., -2., -3.), (5., 2., 9.), (-12., -6., 8.)), // negative vector
        ];

        for (a, b, expected) in cases.iter() {
            let vector_a = Vec3::new(a.0, a.1, a.2);
            let vector_b = Vec3::new(b.0, b.1, b.2);

            assert_eq!(vector_a.cross(vector_b).x(), expected.0);
            assert_eq!(vector_a.cross(vector_b).y(), expected.1);
            assert_eq!(vector_a.cross(vector_b).z(), expected.2);
        }
    }

    #[test]
    fn it_supports_inline_add() {
        let a = Vec3::new(3., 6., 0.4);
        let b = Vec3::new(5., 2., 0.7);
        let expected = Vec3::new(8., 8., 1.1);

        let result = a + b;

        assert_eq!(result.x(), expected.x());
        assert_eq!(result.y(), expected.y());
        assert_eq!(result.z(), expected.z());
    }

    #[test]
    fn it_supports_inline_subtract() {
        let a = Vec3::new(3., 6., 0.4);
        let b = Vec3::new(5., 2., 0.8);
        let expected = Vec3::new(-2., 4., -0.4);

        let result = a - b;

        assert_eq!(result.x(), expected.x());
        assert_eq!(result.y(), expected.y());
        assert_eq!(result.z(), expected.z());
    }

    #[test]
    fn it_supports_inline_mul_vec_vec() {
        let a = Vec3::new(3., 6., 0.4);
        let b = Vec3::new(5., 2., 0.7);
        let expected = Vec3::new(15., 12., 0.28);

        let result = a * b;

        assert_eq!(result.x(), expected.x());
        assert_eq!(result.y(), expected.y());
        assert_eq!(result.z(), expected.z());
    }

    #[test]
    fn it_supports_inline_div_vec_vec() {
        let a = Vec3::new(3., 6., 0.42);
        let b = Vec3::new(3., 2., 0.2);
        let expected = Vec3::new(1., 3., 2.1);

        let result = a / b;

        assert_eq!(result.x(), expected.x());
        assert_eq!(result.y(), expected.y());
        assert_eq!(result.z(), expected.z());
    }

    #[test]
    fn it_supports_inline_div_vec_f32() {
        let a = Vec3::new(3., 6., 9.);
        let b = 3.;

        let expected = Vec3::new(1., 2., 3.);

        let result = a / b;

        assert_eq!(result.x(), expected.x());
        assert_eq!(result.y(), expected.y());
        assert_eq!(result.z(), expected.z());
    }

    #[test]
    fn it_is_comparable() {
        let a = Vec3::new(3., 6., 0.4);
        let b = Vec3::new(3., 6., 0.4);
        let c = Vec3::new(5., 6., 0.4);

        // This works due to Vec3's `PartialEq` derivation
        assert_eq!(a == b, true);
        assert_eq!(a == c, false);
        assert_eq!(b == c, false);
    }
}
