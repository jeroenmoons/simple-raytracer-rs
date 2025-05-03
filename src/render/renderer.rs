use crate::output::image::Image;
use crate::output::output::{Output, OutputType};
use crate::scene::scene::Scene;

pub trait Renderer {
    fn render(
        &mut self,
        scene: &Scene,
        camera_name: String,
        image_w: u32,
        output_type: OutputType,
    ) -> Result<Box<dyn Output>, String>;

    // Calculate a single ray for the specified pixel to see how it travels through the scene
    fn debug_ray(&mut self, x: u32, y: u32, scene: &Scene, camera_name: String, image_w: u32)
    -> ();
}

pub fn get_output(w: u32, h: u32, output_type: &OutputType) -> Box<dyn Output> {
    match output_type {
        OutputType::PNG => Box::from(Image::new(w, h)),
    }
}
