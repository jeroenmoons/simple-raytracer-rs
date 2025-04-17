use crate::math::vector::Point;

// Defines from where a scene is observed, what the camera sees is rendered to the output image
pub struct Camera {
    pub name: String,

    // Position of the camera in the world
    pub center: Point,

    // TODO: Camera direction vector

    // Camera field of vision
    pub focal_length: f32, // Determines distance of the viewport from the camera
    pub w: f32,            // Camera width in world units
    pub aspect_ratio: f32, // Camera aspect ratio, determines height of the camera's view
}

impl Camera {
    pub fn new(name: String, center: Point, focal_length: f32, w: f32, aspect_ratio: f32) -> Self {
        Self {
            name,
            center,
            focal_length,
            w,
            aspect_ratio,
        }
    }
}
