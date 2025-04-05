use crate::math::vector::Color;
use crate::output::output::Output;
use crate::render::renderer::Renderer;
use crate::scene::scene::Scene;

// Renders a hardcoded color gradient to output
pub struct HelloWorld {}

impl HelloWorld {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_pixel(w: f32, h: f32, x: f32, y: f32) -> Color {
        let r = x / w * 255.999;
        let g = y / h * 255.999;
        let b = (x + y) / (w + h) * 255.999;

        Color::new(r, g, b)
    }
}

impl Renderer for HelloWorld {
    fn render(&mut self, _scene: &Scene, w: u32, h: u32, output: &mut dyn Output) -> () {
        println!("Hello world color rendering {w} x {h} image");

        output.init();

        for x in 0..w {
            for y in 0..h {
                let c = Self::calculate_pixel(w as f32, h as f32, x as f32, y as f32);

                output.put_pixel(x, y, &c);
            }
        }

        output.save();
    }
}
