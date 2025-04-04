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
    fn render(&self, w: u32, h: u32, output_image: &String);
}
