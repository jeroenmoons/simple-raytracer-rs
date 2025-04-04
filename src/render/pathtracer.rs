pub struct PathTracer {}

impl PathTracer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, w: &u32, h: &u32) -> () {
        println!("PathTracer rendering {w} x {h} image");
    }
}
