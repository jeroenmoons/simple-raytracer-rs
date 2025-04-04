use crate::output::Output;
use clap::ValueEnum;

pub mod helloworld;
pub mod pathtracer;

#[derive(ValueEnum, Clone, Default, Debug)]
pub enum Algorithm {
    #[default]
    HelloWorld,
    PathTracer,
}

pub trait Renderer {
    fn render(&mut self, w: u32, h: u32, output: &mut dyn Output);
}
