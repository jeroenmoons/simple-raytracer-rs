use crate::output::Output;
use image::{Rgb, RgbImage};
use std::path::Path;

pub struct Image {
    w: u32,
    h: u32,
    path: String,
    buffer: RgbImage,
}

impl Image {
    pub fn new(w: u32, h: u32, path: String) -> Image {
        Self {
            path,
            w,
            h,
            buffer: Default::default(),
        }
    }
}

impl Output for Image {
    fn init(&mut self) -> () {
        self.buffer = RgbImage::new(self.w, self.h)
    }

    fn put_pixel(&mut self, x: u32, y: u32, pixel: Rgb<u8>) {
        self.buffer.put_pixel(x, y, pixel);
    }

    fn save(&mut self) -> () {
        let path = Path::new(&self.path);

        // Save the image to the file system, image format inferred from extension (.png, .jpg, ...)
        self.buffer.save(path).expect("Failed to save image");

        println!("HelloWorld output image saved to {}", path.display());
    }
}
