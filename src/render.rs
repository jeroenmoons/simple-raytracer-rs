use clap::ValueEnum;
use std::fmt::Display;
pub mod helloworld;
pub mod pathtracer;

#[derive(ValueEnum, Clone, Default, Debug)]
pub enum Algorithm {
    #[default]
    HelloWorld,
    PathTracer,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

pub trait Renderer {
    fn render(&self, w: u32, h: u32, output_image: &String);
}
