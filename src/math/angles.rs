use super::constants;

// Convert degrees to radians
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * constants::PI / 180.
}
