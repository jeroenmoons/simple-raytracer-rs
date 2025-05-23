use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::chance::random_f32;

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

const NEAR_ZERO_THRESHOLD: f32 = 1e-8;

impl Vec3 {
    pub fn origin() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn zero() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: glam::Vec3::new(x, y, z),
        }
    }

    pub fn random() -> Self {
        Self::random_minmax(0., 1.)
    }

    pub fn random_minmax(min: f32, max: f32) -> Self {
        Self::new(
            random_f32(min, max),
            random_f32(min, max),
            random_f32(min, max),
        )
    }

    // Return a random Vector of unit length
    pub fn random_unit() -> Self {
        // Retry until random vector is inside the unit sphere -
        // we exclude the corners of the bounding cube, not sure why that is necessary.
        loop {
            let p = Self::random_minmax(-1., 1.);
            let length_squared = p.length_squared();
            // The 1.0e-80 check prevents too short vectors from resulting in a division by zero
            if 1.0e-80 < length_squared && length_squared <= 1. {
                return p / length_squared.sqrt();
            }
        }
    }

    // Generate a random Vector that is located on the same hemisphere as the specified normal vector
    pub fn random_unit_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit();

        if on_unit_sphere.dot(*normal) > 0. {
            return on_unit_sphere; // Random vector is on the same hemisphere as the specified normal
        }

        -on_unit_sphere // Random vector is on the other hemisphere, invert it to bring it into the same hemisphere
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

    pub fn length(&self) -> f32 {
        self.inner.length()
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

    pub fn near_zero(&self) -> bool {
        let s = NEAR_ZERO_THRESHOLD;

        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        Self {
            inner: self.inner - 2. * self.inner.dot(normal.inner) * normal.inner,
        }
    }

    // Calculates refracted vector relative to the normal specified, based on Snell's law - which
    // relates the refractive indices of the material the vector travels through and the material
    // the refracted vector travels through to the angles between these vectors and the normal.
    // `etai_over_etat`: eta for incoming / eta for transmitted
    pub fn refract(&self, normal: &Vec3, etai_over_etat: f32) -> Self {
        // Calculate cos of the angle of the incoming vector with the normal
        let cos_theta = (-self.inner).dot(normal.inner).min(1.);

        // Calculate the components perpendicular and parallel to the normal separately
        let r_out_perp = etai_over_etat * (self.inner + cos_theta * normal.inner);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).max(0.0).sqrt() * normal.inner);

        Self {
            // Sum of the two components constructs the refracted vector
            inner: r_out_perp + r_out_parallel,
        }
    }

    pub fn angle_between(&self, other: Vec3) -> f32 {
        let dot = self.inner.dot(other.inner);
        let mag_product = self.inner.length() * other.inner.length();
        let cos_theta = (dot / mag_product).clamp(-1.0, 1.0);
        cos_theta.acos() // in radians
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

// Support inline subtraction Vec3 + Vec3
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
    fn it_generates_random_vectors() {
        for _ in 0..100 {
            let random_vector = Vec3::random();

            assert_eq!(random_vector.x() >= 0., true);
            assert_eq!(random_vector.x() < 1., true);
            assert_eq!(random_vector.y() >= 0., true);
            assert_eq!(random_vector.y() < 1., true);
            assert_eq!(random_vector.z() >= 0., true);
            assert_eq!(random_vector.z() < 1., true);
        }

        for _ in 0..100 {
            let random_vector = Vec3::random_minmax(5., 6.);

            assert_eq!(random_vector.x() >= 5., true);
            assert_eq!(random_vector.x() < 6., true);
            assert_eq!(random_vector.y() >= 5., true);
            assert_eq!(random_vector.y() < 6., true);
            assert_eq!(random_vector.z() >= 5., true);
            assert_eq!(random_vector.z() < 6., true);
        }

        for _ in 0..100 {
            let random_vector = Vec3::random_minmax(0.3, 0.4);

            assert_eq!(random_vector.x() >= 0.3, true);
            assert_eq!(random_vector.x() < 0.4, true);
            assert_eq!(random_vector.y() >= 0.3, true);
            assert_eq!(random_vector.y() < 0.4, true);
            assert_eq!(random_vector.z() >= 0.3, true);
            assert_eq!(random_vector.z() < 0.4, true);
        }

        for _ in 0..100 {
            let random_vector = Vec3::random_minmax(-10., -9.);

            assert_eq!(random_vector.x() >= -10., true);
            assert_eq!(random_vector.x() < -9., true);
            assert_eq!(random_vector.y() >= -10., true);
            assert_eq!(random_vector.y() < -9., true);
            assert_eq!(random_vector.z() >= -10., true);
            assert_eq!(random_vector.z() < -9., true);
        }
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

    #[test]
    fn it_reflects() {
        let v = Vec3::new(3., -3., 0.);
        let normal = Vec3::new(0., 1., 0.);

        let reflected = v.reflect(&normal);

        assert_eq!(reflected.x(), 3.);
        assert_eq!(reflected.y(), 3.);
        assert_eq!(reflected.z(), 0.);
    }
}
