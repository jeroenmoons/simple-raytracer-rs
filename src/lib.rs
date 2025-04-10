// A `mod` statement is NOT an include or import but a declaration stating that a module exists.
// This `mod` tree maps to the file tree under src to find module implementation code.
pub mod geometry {
    pub mod ray;
    pub mod sphere;
}
pub mod math {
    pub mod vector;
}
pub mod output {
    pub mod image;
    pub mod output;
}
pub mod render {
    pub mod helloworld;
    pub mod pathtracer;
    pub mod renderer;
}
pub mod scene {
    pub mod camera;
    pub mod object;
    pub mod scene;
    pub mod viewport;
}
pub mod scenes {
    pub mod empty;
    pub mod single_orb;
}
