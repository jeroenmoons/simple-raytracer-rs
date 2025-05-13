use crate::math::angles::degrees_to_radians;
use crate::math::vector::{Point, Vec3};
use crate::scene::camera::Camera;

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

        // Calculate the camera's real height in world units from its fixed width and the actual
        // aspect ratio of the target image, which may differ slightly from the ideal aspect ratio
        // the camera defines because pixels are ints and may not have the exact target aspect ratio
        let viewport_aspect_ratio = image_w as f32 / image_h as f32;

        let theta = degrees_to_radians(camera.field_of_view);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * camera.focal_length;
        let viewport_width = viewport_height * viewport_aspect_ratio;

        // Basis vectors for camera coordinate frame
        let basis_w = (camera.look_from - camera.look_at).unit();
        let basis_u = camera.up.cross(basis_w).unit();
        let basis_v = basis_w.cross(basis_u);

        // Vectors spanning width and height of the viewport
        let viewport_u = viewport_width * basis_u;
        let viewport_v = viewport_height * -basis_v;

        // Vectors setting the step size per pixel
        let delta_u = viewport_u / image_w as f32;
        let delta_v = viewport_v / image_h as f32;

        // Steps from the camera's center to the top left of the viewport
        let viewport_origin =
            camera.look_from - (camera.focal_length * basis_w) - viewport_u / 2. - viewport_v / 2.;

        // Position of the first pixel in the viewport (first ray to trace)
        let first_pixel = viewport_origin + 0.5 * (delta_u + delta_v);

        Self {
            image_w,
            image_h,
            w: viewport_width,
            h: viewport_height,
            aspect_ratio: viewport_aspect_ratio,
            first_pixel,
            u: viewport_u,
            v: viewport_v,
            delta_u,
            delta_v,
        }
    }
}
