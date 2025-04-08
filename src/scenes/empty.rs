use crate::scene::scene::Scene;

pub fn generate() -> Scene {
    Scene {
        name: String::from("Empty"),
        objects: vec![],
    }
}
