#[derive(Copy, Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
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

    pub fn clamp(&self, t: f64) -> f64 {
        if t < self.min {
            return self.min;
        } else if t > self.max {
            return self.max;
        }

        t
    }

    pub fn expand(self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}
