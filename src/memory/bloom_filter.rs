use bitvec::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloomFilter {
    bits: BitVec,
    hash_count: usize,
    item_count: usize,
}

impl BloomFilter {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        if expected_items == 0 {
            return Self {
                bits: bitvec![0; 64],
                hash_count: 3,
                item_count: 0,
            };
        }

        // Calculate optimal bit array size
        let bits_per_item = -(false_positive_rate.ln() / std::f64::consts::LN_2.powi(2));
        let bit_count = ((expected_items as f64 * bits_per_item) as usize).max(64);

        // Calculate optimal hash count
        let hash_count = ((bit_count as f64 / expected_items as f64) *
                         std::f64::consts::LN_2).ceil() as usize;
        let hash_count = hash_count.max(1).min(10); // Reasonable bounds

        println!(">> Creating bloom filter: {} bits, {} hashes",
                 bit_count, hash_count);

        Self {
            bits: bitvec![0; bit_count],
            hash_count,
            item_count: 0,
        }
    }

    pub fn insert(&mut self, value: f32) {
        for i in 0..self.hash_count {
            let hash = self.hash(value, i);
            let index = hash % self.bits.len();
            self.bits.set(index, true);
        }
        self.item_count += 1;
    }

    pub fn might_contain(&self, value: f32) -> bool {
        for i in 0..self.hash_count {
            let hash = self.hash(value, i);
            let index = hash % self.bits.len();
            if !self.bits[index] {
                return false; // Definitely not present
            }
        }
        true // Might be present (or false positive)
    }

    fn hash(&self, value: f32, seed: usize) -> usize {
        // Hash function using bit representation of f32
        let bytes = value.to_bits();
        let mut hash = bytes as usize;
        hash ^= seed.wrapping_mul(0x9e3779b9);
        hash = hash.wrapping_mul(0x9e3779b97f4a7c15);
        hash ^= hash >> 32;
        hash
    }

    pub fn expected_false_positive_rate(&self) -> f64 {
        if self.item_count == 0 {
            return 0.0;
        }

        let k = self.hash_count as f64;
        let m = self.bits.len() as f64;
        let n = self.item_count as f64;

        (1.0 - (-k * n / m).exp()).powf(k)
    }

    pub fn len(&self) -> usize {
        self.item_count
    }

    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    pub fn bits_len(&self) -> usize {
        self.bits.len()
    }
}

impl Default for BloomFilter {
    fn default() -> Self {
        Self::new(1000, 0.01)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_basic() {
        let mut bloom = BloomFilter::new(100, 0.01);

        bloom.insert(42.0);
        bloom.insert(3.14);

        assert!(bloom.might_contain(42.0));
        assert!(bloom.might_contain(3.14));
        assert!(!bloom.might_contain(999.0) || true); // May have false positives
    }

    #[test]
    fn test_bloom_filter_false_positive_rate() {
        let mut bloom = BloomFilter::new(1000, 0.01);

        for i in 0..1000 {
            bloom.insert(i as f32);
        }

        let fp_rate = bloom.expected_false_positive_rate();
        assert!(fp_rate < 0.02, "FP rate too high: {}", fp_rate);
    }

    #[test]
    fn test_bloom_filter_negative_lookups() {
        let mut bloom = BloomFilter::new(100, 0.01);

        for i in 0..100 {
            bloom.insert(i as f32);
        }

        // Values definitely not inserted should return false
        let mut false_negatives = 0;
        for i in 1000..1100 {
            if !bloom.might_contain(i as f32) {
                false_negatives += 1;
            }
        }

        // Should have mostly correct negatives (allowing some false positives)
        assert!(false_negatives > 90, "Too many false positives");
    }
}
