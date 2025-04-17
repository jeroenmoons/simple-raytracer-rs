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
            min: f32::INFINITY,
            max: -f32::INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -f32::INFINITY,
            max: f32::INFINITY,
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

    pub fn clamp(&self, x: f32) -> f32 {
        if x <= self.min {
            return self.min;
        }
        if x >= self.max {
            return self.max;
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_should_clamp_correctly() {
        let cases = [
            ((1., 2.), 3., 2.),
            ((1., 2.), 0., 1.),
            ((1., 2.), 1.5, 1.5),
            ((1., 2.), 1., 1.),
            ((1., 2.), 2., 2.),
        ];

        for ((min, max), x, expected) in cases.iter() {
            let interval = Interval::new(*min, *max);

            assert_eq!(interval.clamp(*x), *expected);
        }
    }
}
