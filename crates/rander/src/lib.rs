//! this Random Number Generator implmements SplitMix64 algorithm

use std::hash::{BuildHasher, Hasher, RandomState};

pub struct Rng {
    state: u64,
}

impl Rng {
    #[must_use]
    /// Create a new RNG with a specific seed (useful for reproducible renders).
    pub const fn seed_from_u64(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub const fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    #[must_use]
    pub const fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x_9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0x_BF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x_94D0_49BB_1331_11EB);

        z ^ (z >> 31)
    }

    #[inline]
    /// # Panics
    /// it panics if low > high.
    pub fn range_u64(&mut self, low: u64, high: u64) -> u64 {
        assert!(low < high);
        let span = high - low;

        // Rejection sampling to avoid modulo bias.
        let zone = u64::MAX - (u64::MAX % span);
        loop {
            let v = self.next_u64();
            if v < zone {
                return low + v % span;
            }
        }
    }

    #[inline]
    /// Returns a random f64 in [0, 1). Essential for ray tracing.
    pub fn next_f64(&mut self) -> f64 {
        // Use the top 53 bits to fill the mantissa of an f64
        (self.next_u64() >> 11) as f64 * (1.0 / (1_u64 << 53) as f64)
    }

    #[inline]
    /// Returns a random f64 in [min, max).
    /// Built on `next_f64()` the tiny float bias is negligible for rendering.
    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        debug_assert!(min < max);
        min + (max - min) * self.next_f64()
    }

    pub fn shuffle<T>(&mut self, xs: &mut [T]) {
        for i in (1..xs.len()).rev() {
            let j = self.range_u64(0, i as u64 + 1) as usize;
            xs.swap(i, j);
        }
    }

    #[must_use]
    /// Split into a new, independent RNG.
    /// Use this to give each thread its own non-overlapping random stream.
    pub const fn split(&mut self) -> Self {
        Self {
            state: self.next_u64(),
        }
    }
}

impl Default for Rng {
    /// Create a new RNG seeded from system entropy.
    fn default() -> Self {
        let mut h = RandomState::new().build_hasher();
        h.write_u64(0);
        Self { state: h.finish() }
    }
}
