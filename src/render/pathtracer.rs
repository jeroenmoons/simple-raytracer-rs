use crate::math::vector::Color;
use crate::output::Output;
use crate::render::Renderer;
use crate::scene::scene::Scene;
use std::io;
use std::io::Write;

pub struct PathTracer {}

impl PathTracer {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_pixel(w: f32, h: f32, x: f32, y: f32) -> Color {
        let r = x / w * 255.;
        let g = y / h * 255.;
        let b = (x + y) / (w + h) * 255.;

        Color::new(r, g, b)
    }

    fn print_progress(total_pixels: usize, count: usize) {
        if count % 1000 == 0 || count == total_pixels {
            let percent = count as f32 / total_pixels as f32 * 100.0;
            print!("\rRendering: {:>5.1}%", percent);
            io::stdout().flush().unwrap();
        }
    }
}

impl Renderer for PathTracer {
    fn render(&mut self, scene: &Scene, w: u32, h: u32, output: &mut dyn Output) -> () {
        println!(
            "PathTracer rendering scene {} into a {w} x {h} image",
            scene.name
        );

        output.init();

        let total_pixels = (w * h) as usize;
        let mut count = 0;

        for x in 0..w {
            for y in 0..h {
                count += 1;

                Self::print_progress(total_pixels, count);

                let color = Self::calculate_pixel(w as f32, h as f32, x as f32, y as f32);

                output.put_pixel(x, y, &color);
            }
        }

        println!(" -> Done!");

        output.save()
    }
}
