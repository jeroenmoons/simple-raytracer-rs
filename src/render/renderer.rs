use crate::output::output::Output;
use crate::scene::scene::Scene;

pub trait Renderer {
    fn render(&mut self, scene: &Scene, w: u32, h: u32, output: &mut dyn Output);
}
