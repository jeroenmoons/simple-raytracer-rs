// A `mod` statement is NOT an include or import but a declaration stating that a module exists.
// This `mod` tree maps to the file tree under src to find module implementation code.
pub mod geometry {
    pub mod ray;
    pub mod sphere;
}
pub mod material {
    pub mod base;
    pub mod diffuse;
}
pub mod math {
    pub mod angles;
    pub mod chance;
    pub mod constants;
    pub mod numbers;
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
    pub mod diffuse_orb;
    pub mod empty;
    pub mod lambert_orb;
    pub mod orb_with_ground_lambert;
    pub mod small_orb_in_front_of_larger_one;
}
