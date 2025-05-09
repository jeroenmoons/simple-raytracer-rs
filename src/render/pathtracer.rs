use crate::geometry::ray::Ray;
use crate::math::chance::random_f32;
use crate::math::vector::{Color, Vec3};
use crate::output::output::{Output, OutputType};
use crate::render::renderer::{Renderer, get_output};
use crate::scene::camera::Camera;
use crate::scene::scene::Scene;
use crate::scene::viewport::Viewport;
use std::io;
use std::io::Write;

// Renders a Scene to output using a path tracing algorithm
pub struct PathTracer {
    samples_per_pixel: u32,
    pixel_samples_scale: f32,
    max_depth: u32, // Maximum number of ray bounces into scene
}

impl PathTracer {
    pub fn new(samples_per_pixel: u32, max_depth: u32) -> Self {
        Self {
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            max_depth,
        }
    }

    fn get_ray(&self, camera: &Camera, viewport: &Viewport, x: u32, y: u32) -> Ray {
        let offset = self.sample_square();

        // Get the center location of the pixel on the viewport plane to calculate its color
        let pixel = viewport.first_pixel
            + ((x as f32 + offset.x()) * viewport.delta_u)
            + ((y as f32 + offset.y()) * viewport.delta_v);

        // Construct a ray originating at the camera center pointed towards the pixel we are rendering
        Ray::from_to(camera.center, pixel)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f32(0., 1.) - 0.5, random_f32(0., 1.) - 0.5, 0.)
    }

    fn calculate_pixel(&self, scene: &Scene, ray: &Ray, depth: u32) -> Color {
        ray_debug!("Depth {} - calculating pixel", depth);

        if depth <= 0 {
            ray_debug!("Depth {} - max depth, color black", depth);

            // Max depth reached, stop tracing
            return Color::zero(); // Contribute no more light to the pixel
        }

        match scene.trace(ray) {
            (Some(obj), Some(hit)) => match obj.material().scatter(&ray, &hit) {
                Some((scatter, attenuation)) => {
                    ray_debug!(
                        "Depth {} - ray scattered to {} at an angle of {}°, current attenuation {}",
                        depth,
                        scatter,
                        ray.direction.angle_between(scatter.direction).to_degrees(),
                        attenuation
                    );

                    attenuation * self.calculate_pixel(scene, &scatter, depth - 1)
                }
                None => {
                    ray_debug!("Depth {} - ray not scattered, returning black", depth);

                    Color::zero() // No scattering, do not contribute any light}
                }
            },
            _ => {
                ray_debug!("NO HIT - USING BACKGROUND");

                // Nothing was hit, fall back to background gradient
                let unit_direction = ray.direction.unit();
                let a = 0.5 * unit_direction.y() + 1.0;

                // Blend (or "lerp", linear interpolation) of white and blue based on the ray's Y coordinate
                (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
            }
        }
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
        let image_h = viewport.image_h; // Height is determined from specified width and camera aspect ratio

        let mut output = get_output(image_w, image_h, &output_type);

        println!(
            "PathTracer rendering {} to a {image_w} x {image_h} {:?} image",
            scene.name, output_type,
        );

        println!("Viewport: {viewport:#?}");

        output.init();

        let total_pixels = (image_w * image_h) as usize;
        let mut count = 0;

        for x in 0..image_w {
            for y in 0..image_h {
                count += 1;
                Self::print_progress(total_pixels, count);

                let mut color = Color::zero();

                // We sample a number of rays for the same pixel and use the average color. This
                // implements antialiasing.
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(&camera, &viewport, x, y);

                    // Simple line where the bulk of the complexity lies: find out which color the
                    // pixel should have based on the Scene geometry, lights, materials, ...
                    color = color + self.calculate_pixel(scene, &ray, self.max_depth);
                }

                // We have added colors for all samples, now we calculate the average
                color = color * self.pixel_samples_scale;

                output.put_pixel(x, y, &color);
            }
        }

        println!(" -> Done!");

        Ok(output)
    }

    fn debug_ray(
        &mut self,
        x: u32,
        y: u32,
        scene: &Scene,
        camera_name: String,
        image_w: u32,
    ) -> () {
        let camera = scene.get_camera(&camera_name).unwrap();
        let viewport = Viewport::from(camera, image_w);

        ray_debug!(
            "PathTracer debugging of ray at {},{} in scene {} - {} x {} image",
            x,
            y,
            scene.name,
            image_w,
            viewport.image_h,
        );

        let ray = self.get_ray(&camera, &viewport, x, y);

        ray_debug!("Tracing ray {}", ray);

        println!(
            "Pixel color: {:?}",
            self.calculate_pixel(scene, &ray, self.max_depth)
        );
    }
}
