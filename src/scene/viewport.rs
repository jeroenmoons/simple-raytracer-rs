// In-scene representation of the rendered image.
// Located in front of the Camera center at the focal length distance.
pub struct Viewport {
    // Target image resolution in pixels
    pub image_w: u32,
    pub image_h: u32,

    // Viewport size
    pub w: f32, // Width in world units
    pub h: f32, // Height in world units
    pub aspect_ratio: f32,
}

impl Viewport {
    // Create a new viewport with the specified height and a width based on the target image's aspect ratio
    pub fn new(viewport_h: f32, image_w: u32, image_h: u32) -> Self {
        let aspect_ratio = image_w as f32 / image_h as f32;
        let viewport_w = viewport_h * aspect_ratio;

        Self {
            w: viewport_w,
            h: viewport_h,
            aspect_ratio,
            image_w,
            image_h,
        }
    }
}
