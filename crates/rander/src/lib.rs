use std::hash::{BuildHasher, Hasher, RandomState};

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new() -> Self {
        let mut h = RandomState::new().build_hasher();
        h.write_u64(0);
        Self { state: h.finish() }
    }

    pub fn next_u64(&mut self) -> u64 {
        // SplitMix64
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

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

    pub fn shuffle<T>(&mut self, xs: &mut [T]) {
        for i in (1..xs.len()).rev() {
            let j = self.range_u64(0, i as u64 + 1) as usize;
            xs.swap(i, j);
        }
    }
}
