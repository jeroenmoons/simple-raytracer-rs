use crate::math::vector::{Point, Vec3};
use crate::scene::camera::Camera;
use std::fmt;
use std::fmt::Display;

// In-scene representation of the rendered image.
// Located in front of the Camera center at the focal length distance.
#[derive(Debug)]
pub struct Viewport {
    // Target image resolution in pixels
    pub image_w: u32,
    pub image_h: u32,

    // Viewport size
    pub w: f32, // Width in world units
    pub h: f32, // Height in world units
    pub aspect_ratio: f32,

    // Location of the top left pixel of the viewport
    pub first_pixel: Point,

    // Vectors u and v are spanning width and height of the viewport
    pub u: Vec3,
    pub v: Vec3,

    // Vectors delta_u and delta_v define the step size between pixels on the viewport plane
    pub delta_u: Vec3,
    pub delta_v: Vec3,
}

impl Viewport {
    pub fn from(camera: &Camera, image_w: u32) -> Self {
        // Calculate the final image height from the specified target width and the camera's aspect ratio
        let image_h = (image_w as f32 / camera.aspect_ratio).round() as u32;

        // Camera's width in world units is fixed, a defining parameter of the camera
        let w = camera.w;

        // Calculate the camera's real height in world units from its fixed width and the actual
        // aspect ratio of the target image, which may differ slightly from the ideal aspect ratio
        // the camera defines because pixels are ints and may not have the exact target aspect ratio
        let viewport_aspect_ratio = image_w as f32 / image_h as f32;
        let h = w / viewport_aspect_ratio;

        let u = Vec3::new(w, 0., 0.); // TODO: Dynamic camera location and direction
        let v = Vec3::new(0., -h, 0.); // TODO: Dynamic camera location and direction

        let delta_u = u / image_w as f32;
        let delta_v = v / image_h as f32;

        let focal_length_vector = Point::new(0., 0., camera.focal_length);

        // Steps from the camera's center to the top left of the viewport
        let viewport_origin = camera.center - focal_length_vector - u / 2. - v / 2.;

        let first_pixel = viewport_origin + 0.5 * (delta_u + delta_v);

        Self {
            image_w,
            image_h,
            w,
            h,
            aspect_ratio: viewport_aspect_ratio,
            first_pixel,
            u,
            v,
            delta_u,
            delta_v,
        }
    }
}

impl Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Viewport(\
                image_w {}, \
                image_h {}, \
                w {}, \
                h {}, \
                ar {}, \
                first pixel {}, \
                u {}, \
                v {}, \
                delta_u {}, \
                delta_v {}\
            )",
            self.image_w,
            self.image_h,
            self.w,
            self.h,
            self.aspect_ratio,
            self.first_pixel,
            self.u,
            self.v,
            self.delta_u,
            self.delta_v,
        )
    }
}
