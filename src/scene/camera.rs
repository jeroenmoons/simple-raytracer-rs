use crate::math::vector::{Point, Vec3};

use super::viewport::Viewport;

// Defines from where a scene is observed, what the camera sees is rendered to the output image
pub struct Camera {
    pub center: Point,
    pub focal_length: f32,
    pub viewport: Viewport,
}

impl Camera {
    pub fn new(center: Point, focal_length: f32, viewport: Viewport) -> Self {
        Self {
            center,
            focal_length,
            viewport,
        }
    }

    // Get vector pointing to the top left pixel of the camera's viewport.
    pub fn get_first_pixel(&self) -> Point {
        let focal_length_vector = Point::new(0., 0., self.focal_length);

        // Steps from the camera's center to the top left of the viewport
        let viewport_origin = self.center - focal_length_vector - self.u() / 2. - self.v() / 2.;

        viewport_origin + 0.5 * (self.delta_u() + self.delta_v())
    }

    // Vectors u and v are spanning width and height of the viewport
    pub fn u(&self) -> Vec3 {
        Vec3::new(self.viewport.w, 0., 0.) // TODO: dynamic camera orientation
    }
    pub fn v(&self) -> Vec3 {
        Vec3::new(0., -self.viewport.h, 0.) // TODO: dynamic camera orientation
    }

    // Vectors delta_u and delta_v define the step size between pixels on the viewport plane
    pub fn delta_u(&self) -> Vec3 {
        self.u() / self.viewport.image_w as f32
    }
    pub fn delta_v(&self) -> Vec3 {
        self.v() / self.viewport.image_h as f32
    }
}
