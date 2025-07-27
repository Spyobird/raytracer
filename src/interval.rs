use crate::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY_INTERVAL
    }
}
