use crate::math::vector::Color;
use std::str::FromStr;

// Represents a generic output of a rendering algorithm
pub trait Output {
    fn init(&mut self) -> ();
    fn put_pixel(&mut self, x: u32, y: u32, c: &Color);
    fn save(&self, save_path: &String) -> ();
}

#[derive(Debug)]
pub enum OutputType {
    PNG,
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(OutputType::PNG),
            _ => Err(format!("Unknown output type: {}", s)),
        }
    }
}
