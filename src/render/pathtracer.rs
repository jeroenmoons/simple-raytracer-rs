use crate::geometry::ray::Ray;
use crate::math::vector::{Color, Point};
use crate::output::output::Output;
use crate::render::renderer::Renderer;
use crate::scene::camera::Camera;
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
        if scene.hit_by(ray) {
            // Object in the scene was hit
            return Color::new(0.5, 0.1, 0.1);
        }

        // Nothing was hit, fall back to background gradient
        let unit_direction = ray.direction.unit(); // Ray direction as a vector of length 1
        let a = 0.5 * unit_direction.y() + 1.0;

        // Blend ("lerp", linear interpolation) between white and blue colors based on the ray's Y coordinate
        (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
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
        println!("PathTracer rendering {} to a {w} x {h} image", scene.name);

        output.init();

        let total_pixels = (w * h) as usize;
        let mut count = 0;

        let viewport = Viewport::new(2., w, h);

        let camera = Camera::new(Point::origin(), 1.0, viewport);

        // Determine scan origin and step sizes once up front.
        // Camera position should be fixed for a single frame so we can avoid doing this for every pixel.
        let first_pixel = camera.get_first_pixel();
        let delta_u = camera.delta_u();
        let delta_v = camera.delta_v();

        for x in 0..w {
            for y in 0..h {
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

        output.save()
    }
}
