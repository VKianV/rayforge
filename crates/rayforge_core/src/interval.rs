#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    /// Default interval is empty.
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    /// The half-open ray range: from the camera plane outward to infinity.
    pub const NON_NEGATIVE: Self = Self {
        min: 0.0,
        max: f32::INFINITY,
    };

    #[must_use]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    #[must_use]
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    #[must_use]
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    #[must_use]
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
