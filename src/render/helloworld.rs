use crate::math::vector::Color;
use crate::output::output::{Output, OutputType};
use crate::render::renderer::{Renderer, get_output};
use crate::scene::scene::Scene;
use crate::scene::viewport::Viewport;

// Renders a hardcoded color gradient to output
pub struct HelloWorld {}

impl HelloWorld {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_pixel(w: f32, h: f32, x: f32, y: f32) -> Color {
        let r = x / w;
        let g = y / h;
        let b = (x + y) / (w + h);

        let color = Color::new(r, g, b);

        println!("Writing pixel {x},{y} to image {w}x{h}: {}", color);

        color
    }
}

impl Renderer for HelloWorld {
    fn render(
        &mut self,
        scene: &Scene,
        camera_name: String,
        image_w: u32,
        output_type: OutputType,
    ) -> Result<Box<dyn Output>, String> {
        let camera = scene
            .get_camera(&camera_name)
            .ok_or(format!("Camera {camera_name} not found"))?;

        let viewport = Viewport::from(camera, image_w);
        let image_h = viewport.image_h;

        let mut output = get_output(image_w, image_h, &output_type);

        println!("Hello world color rendering {image_w} x {image_h} image");

        output.init();

        for x in 0..image_w {
            for y in 0..image_h {
                let c = Self::calculate_pixel(image_w as f32, image_h as f32, x as f32, y as f32);

                output.put_pixel(x, y, &c);
            }
        }

        Ok(output)
    }
}
