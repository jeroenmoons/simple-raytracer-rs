use super::output::Output;
use crate::math::numbers::Interval;
use crate::math::vector::Color;
use image::{Rgb, RgbImage};
use std::path::Path;

// Generates an output image using Rust's built-in image module
pub struct Image {
    w: u32,
    h: u32,
    buffer: Option<RgbImage>,
    intensity: Interval,
}

impl Image {
    pub fn new(w: u32, h: u32) -> Image {
        Self {
            w,
            h,
            buffer: None,
            intensity: Interval::new(0.0, 0.999),
        }
    }

    // Transforms a color component from linear space to gamma space (gamma correction)
    fn linear_to_gamma(&mut self, linear_component: f32) -> f32 {
        if linear_component > 0. {
            return linear_component.sqrt();
        }

        0.
    }

    //     inline double linear_to_gamma(double linear_component)
    // {
    //     if (linear_component > 0)
    //         return std::sqrt(linear_component);

    //     return 0;
    // }
}

impl Output for Image {
    fn init(&mut self) -> () {
        self.buffer = Some(RgbImage::new(self.w, self.h));
    }

    fn put_pixel(&mut self, x: u32, y: u32, c: &Color) {
        let r = self.linear_to_gamma(c.x());
        let g = self.linear_to_gamma(c.y());
        let b = self.linear_to_gamma(c.z());

        if let Some(buffer) = self.buffer.as_mut() {
            buffer.put_pixel(
                x,
                y,
                Rgb([
                    (self.intensity.clamp(r) * 255.) as u8,
                    (self.intensity.clamp(g) * 255.) as u8,
                    (self.intensity.clamp(b) * 255.) as u8,
                ]),
            );
        } else {
            panic!("Pixel buffer is not set, call init first");
        }
    }

    fn save(&self, save_path: &String) -> () {
        let path = Path::new(save_path);

        match self.buffer {
            Some(ref buffer) => {
                buffer.save(path).expect("Failed to save image");
                println!("Output image saved at {}", path.display());
            }
            None => println!("Buffer not set, nothing to save"),
        }
    }
}
