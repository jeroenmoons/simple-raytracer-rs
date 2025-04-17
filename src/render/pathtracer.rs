use crate::geometry::ray::Ray;
use crate::math::vector::Color;
use crate::output::output::{Output, OutputType};
use crate::render::renderer::{Renderer, get_output};
use crate::scene::scene::Scene;
use crate::scene::viewport::Viewport;
use std::io;
use std::io::Write;

// Renders a Scene to output using a path tracing algorithm
pub struct PathTracer {}

impl PathTracer {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_pixel(scene: &Scene, ray: &Ray) -> Color {
        match scene.trace(ray) {
            Some(color) => color, // Something in the scene determined the pixel's color
            _ => {
                // Nothing was hit, fall back to background gradient
                let unit_direction = ray.direction.unit(); // Ray direction as a vector of length 1
                let a = 0.5 * unit_direction.y() + 1.0;

                // Blend ("lerp", linear interpolation) between white and blue colors based on the ray's Y coordinate
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
        let image_h = viewport.image_h;

        let mut output = get_output(image_w, image_h, &output_type);

        println!(
            "PathTracer rendering {} to a {image_w} x {image_h} {:?} image",
            scene.name, output_type,
        );

        println!("Viewport: {viewport}");

        output.init();

        let total_pixels = (image_w * image_h) as usize;
        let mut count = 0;

        // Determine scan origin and step sizes once up front.
        // Camera position should be fixed for a single frame so we can avoid doing this for every pixel.
        let first_pixel = viewport.first_pixel;
        let delta_u = viewport.delta_u;
        let delta_v = viewport.delta_v;

        for x in 0..image_w {
            for y in 0..image_h {
                count += 1;

                Self::print_progress(total_pixels, count);

                // Get the center location of the pixel on the viewport plane to calculate its color
                let pixel = first_pixel + (x as f32 * delta_u) + (y as f32 * delta_v);

                // Construct a ray originating at the camera center pointed towards the pixel we are rendering
                let ray = Ray::new(camera.center, pixel);

                // Simple line where the bulk of the complexity lies: find out which color the pixel should have based
                // on the Scene geometry, lights, materials, ...
                let color = Self::calculate_pixel(&scene, &ray);

                output.put_pixel(x, y, &color);
            }
        }

        println!(" -> Done!");

        Ok(output)
    }
}
