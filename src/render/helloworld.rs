use crate::output::Output;
use crate::render::Renderer;
use crate::scene::scene::Scene;
use image::Rgb;

pub struct HelloWorld {}

impl HelloWorld {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_pixel(w: f32, h: f32, x: f32, y: f32) -> (u8, u8, u8) {
        let r = (x / w * 255.999) as u8;
        let g = (y / h * 255.999) as u8;
        let b = ((x + y) / (w + h) * 255.999) as u8;

        (r, g, b)
    }
}

impl Renderer for HelloWorld {
    fn render(&mut self, _scene: &Scene, w: u32, h: u32, output: &mut dyn Output) -> () {
        println!("Hello world color rendering {w} x {h} image");

        output.init();

        for x in 0..w {
            for y in 0..h {
                let (r, g, b) = Self::calculate_pixel(w as f32, h as f32, x as f32, y as f32);

                output.put_pixel(x, y, Rgb([r, g, b]));
            }
        }

        output.save();
    }
}
