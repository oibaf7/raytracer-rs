pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && self.max >= t
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min < t && self.max > t
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY
        }
    }
}