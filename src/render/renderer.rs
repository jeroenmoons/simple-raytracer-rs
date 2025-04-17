use crate::output::image::Image;
use crate::output::output::{Output, OutputType};
use crate::scene::scene::Scene;

pub trait Renderer {
    fn render(
        &mut self,
        scene: &Scene,
        camera_name: String,
        width: u32,
        output_type: OutputType,
    ) -> Result<Box<dyn Output>, String>;
}

pub fn get_output(w: u32, h: u32, output_type: &OutputType) -> Box<dyn Output> {
    match output_type {
        OutputType::PNG => Box::from(Image::new(w, h)),
    }
}
