use crate::output::Output;
use image::{Rgb, RgbImage};
use std::path::Path;
use std::time::Instant;

pub struct Image {
    w: u32,
    h: u32,
    path: String,
    buffer: Option<RgbImage>,
    init_instant: Option<Instant>,
}

impl Image {
    pub fn new(w: u32, h: u32, path: String) -> Image {
        Self {
            path,
            w,
            h,
            buffer: None,
            init_instant: None,
        }
    }
}

impl Output for Image {
    fn init(&mut self) -> () {
        self.buffer = Some(RgbImage::new(self.w, self.h));
        self.init_instant = Some(Instant::now());
    }

    fn put_pixel(&mut self, x: u32, y: u32, pixel: Rgb<u8>) {
        if let Some(buffer) = self.buffer.as_mut() {
            buffer.put_pixel(x, y, pixel);
        } else {
            panic!("Pixel buffer is not set, call init first");
        }
    }

    fn save(&mut self) -> () {
        let path = Path::new(&self.path);

        match self.buffer {
            Some(ref mut buffer) => {
                buffer.save(path).expect("Failed to save image");

                self.init_instant
                    .map(|s| println!("Render completed in {:.2?}", s.elapsed()));

                println!("Output image saved at {}", path.display());
            }
            None => {
                println!("Buffer not set, nothing to save");
            }
        }
    }
}
