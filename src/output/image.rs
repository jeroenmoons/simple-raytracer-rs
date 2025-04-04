use super::output::Output;
use crate::math::vector::Color;
use image::{Rgb, RgbImage};
use std::path::Path;

// Generates an output image using Rust's built-in image module
pub struct Image {
    w: u32,
    h: u32,
    path: String,
    buffer: Option<RgbImage>,
}

impl Image {
    pub fn new(w: u32, h: u32, path: String) -> Image {
        Self {
            path,
            w,
            h,
            buffer: None,
        }
    }
}

impl Output for Image {
    fn init(&mut self) -> () {
        self.buffer = Some(RgbImage::new(self.w, self.h));
    }

    fn put_pixel(&mut self, x: u32, y: u32, c: &Color) {
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.put_pixel(x, y, Rgb([c.x() as u8, c.y() as u8, c.z() as u8]));
        } else {
            panic!("Pixel buffer is not set, call init first");
        }
    }

    fn save(&mut self) -> () {
        let path = Path::new(&self.path);

        match self.buffer {
            Some(ref mut buffer) => {
                buffer.save(path).expect("Failed to save image");
                println!("Output image saved at {}", path.display());
            }
            None => println!("Buffer not set, nothing to save"),
        }
    }
}
