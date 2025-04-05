use crate::math::vector::Color;

pub mod image;

pub trait Output {
    fn init(&mut self) -> ();
    fn put_pixel(&mut self, x: u32, y: u32, c: &Color);
    fn save(&mut self) -> ();
}
