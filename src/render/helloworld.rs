use crate::render::Renderer;
use image::{Rgb, RgbImage};
use std::path::Path;

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

    fn save_image(output_image: &String, img: &mut RgbImage) {
        let path = Path::new(output_image);

        // Save the image to the file system, image format inferred from extension (.png, .jpg, ...)
        img.save(path).expect("Failed to save image");

        println!("HelloWorld output image saved to {}", path.display());
    }

    fn create_blank_image(w: &u32, h: &u32) -> RgbImage {
        RgbImage::new(*w, *h)
    }
}

impl Renderer for HelloWorld {
    fn render(&self, w: u32, h: u32, output_image: &String) -> () {
        println!("Hello world color rendering {w} x {h} image");

        // TODO - abstraction layer for the output
        let mut img = Self::create_blank_image(&w, &h);

        for x in 0..w {
            for y in 0..h {
                let (r, g, b) = Self::calculate_pixel(w as f32, h as f32, x as f32, y as f32);

                // Set the pixel color at (x, y)
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }

        Self::save_image(output_image, &mut img);
    }
}
