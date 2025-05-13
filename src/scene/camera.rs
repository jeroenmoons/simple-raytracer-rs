use crate::math::vector::{Point, Vec3};

// Defines from where a scene is observed, what the camera sees is rendered to the output image
pub struct Camera {
    pub name: String,

    pub look_from: Point, // Position of the camera in the world
    pub look_at: Point,   // Point towards which the camera is aimed
    pub up: Vec3, // Points towards the top of the camera (manipulating this would rotate the camera)

    pub field_of_view: f32, // Vertical view angle in degrees - angle at look_from between top and bottom edge of viewport

    // Camera field of vision
    pub focal_length: f32, // Determines distance of the viewport from the camera
    pub aspect_ratio: f32, // Camera aspect ratio, determines height of the camera's view
}

impl Camera {
    pub fn new(
        name: String,
        look_from: Point,
        look_at: Point,
        field_of_view: f32,
        aspect_ratio: f32,
    ) -> Self {
        Self {
            name,
            look_from,
            look_at,
            up: Vec3::new(0., 1., 0.), // Fixed for now
            field_of_view,
            focal_length: (look_from - look_at).length(),
            aspect_ratio,
        }
    }
}
