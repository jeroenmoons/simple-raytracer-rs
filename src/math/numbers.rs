#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: core::f32::INFINITY,
            max: -core::f32::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -core::f32::INFINITY,
            max: core::f32::INFINITY,
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, n: f32) -> bool {
        self.min <= n && n <= self.max
    }

    pub fn surrounds(&self, n: f32) -> bool {
        self.min < n && n < self.max
    }
}
