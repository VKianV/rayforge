use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<T = f32> {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T>
where
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> T {
        self.max - self.min
    }

    pub fn map_to(&self, sample: T, dist_min: T, dist_max: T) -> T {
        dist_min + (dist_max - dist_min) * (sample - self.min) / (self.max - self.min)
    }

    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: T) -> T {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Interval<f32> {
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    pub const NON_NEGATIVE: Self = Self {
        min: 0.0,
        max: f32::INFINITY,
    };
}

impl Default for Interval<f32> {
    fn default() -> Self {
        Self::EMPTY
    }
}
