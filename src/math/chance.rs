use rand::Rng;

pub fn random_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chance() {
        let random_f32 = random_f32(0.0, 1.0);

        assert!(random_f32 < 1.0);
        assert!(random_f32 >= 0.0);
    }
}
