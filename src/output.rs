use ::image::Rgb;

pub mod image;

pub trait Output {
    fn init(&mut self) -> ();
    fn put_pixel(&mut self, x: u32, y: u32, pixel: Rgb<u8>);
    fn save(&mut self) -> ();
}
