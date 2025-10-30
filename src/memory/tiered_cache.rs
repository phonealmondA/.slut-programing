use lru::LruCache;
use std::num::NonZeroUsize;
use std::collections::HashMap;
use anyhow::Result;

use super::binary_cache::BinaryCache;
use super::bloom_filter::BloomFilter;
use super::partitioned_index::PartitionedIndex;
use super::compact_solution::CompactSolution;
use crate::MathSolution;

pub struct TieredMemory {
    // L1: Hot cache (100 most recent, in RAM)
    hot_cache: LruCache<u32, CompactSolution>,

    // L2: Warm cache (1000 frequently used, compressed)
    warm_cache: Vec<CompactSolution>,
    warm_index: HashMap<u32, usize>,

    // L3: Cold storage (everything else, on disk)
    cold_storage: BinaryCache,

    // Fast index for cold storage
    cold_index: PartitionedIndex,

    // Bloom filter for fast rejection
    bloom: BloomFilter,

    // Metrics
    hot_hits: u64,
    warm_hits: u64,
    cold_hits: u64,
    misses: u64,
}

impl TieredMemory {
    pub fn new(cold_storage: BinaryCache) -> Self {
        // Build index on initialization
        let cold_index = PartitionedIndex::build_from_solutions(
            &cold_storage.solutions
        );

        // Build bloom filter from cold storage
        let mut bloom = BloomFilter::new(
            cold_storage.solutions.len().max(1),
            0.01  // 1% false positive rate
        );

        for solution in &cold_storage.solutions {
            bloom.insert(solution.result);
        }

        println!(">> Bloom filter FP rate: {:.2}%",
                 bloom.expected_false_positive_rate() * 100.0);

        Self {
            hot_cache: LruCache::new(NonZeroUsize::new(100).unwrap()),
            warm_cache: Vec::with_capacity(1000),
            warm_index: HashMap::new(),
            cold_storage,
            cold_index,
            bloom,
            hot_hits: 0,
            warm_hits: 0,
            cold_hits: 0,
            misses: 0,
        }
    }

    pub fn get_solution(&mut self, target: f64) -> Option<MathSolution> {
        let target_f32 = target as f32;
        let target_key = (target * 100.0) as u32;

        // FIRST: Check bloom filter (< 0.1 microseconds)
        if !self.bloom.might_contain(target_f32) {
            self.misses += 1;
            return None; // 100% certain it's not there
        }

        // Try L1 hot cache (fastest)
        if let Some(compact) = self.hot_cache.get(&target_key) {
            self.hot_hits += 1;
            return Some(compact.to_math_solution(
                &self.cold_storage.operand_pool,
                self.cold_storage.start_time
            ));
        }

        // Try L2 warm cache
        if let Some(&index) = self.warm_index.get(&target_key) {
            if index < self.warm_cache.len() {
                let compact = &self.warm_cache[index];
                self.warm_hits += 1;

                // Promote to hot cache
                self.hot_cache.put(target_key, compact.clone());

                return Some(compact.to_math_solution(
                    &self.cold_storage.operand_pool,
                    self.cold_storage.start_time
                ));
            }
        }

        // Try L3 cold storage with fast index
        if let Some(index) = self.cold_index.smart_search(target_f32) {
            if index < self.cold_storage.solutions.len() {
                let compact = &self.cold_storage.solutions[index];
                self.cold_hits += 1;

                let solution = compact.to_math_solution(
                    &self.cold_storage.operand_pool,
                    self.cold_storage.start_time
                );

                // Promote to warm cache
                self.promote_to_warm(target_key, compact.clone());

                return Some(solution);
            }
        }

        self.misses += 1;
        None
    }

    fn promote_to_warm(&mut self, key: u32, solution: CompactSolution) {
        if self.warm_cache.len() >= 1000 {
            // Evict oldest from warm cache (simple FIFO)
            self.warm_cache.remove(0);
            // Rebuild index (simple approach)
            self.warm_index.clear();
            for (i, sol) in self.warm_cache.iter().enumerate() {
                let k = (sol.result * 100.0) as u32;
                self.warm_index.insert(k, i);
            }
        }

        let index = self.warm_cache.len();
        self.warm_cache.push(solution);
        self.warm_index.insert(key, index);
    }

    pub fn insert_solution(&mut self, solution: MathSolution) {
        let target_key = (solution.result * 100.0) as u32;
        let target_f32 = solution.result as f32;

        // Update bloom filter
        self.bloom.insert(target_f32);

        // Add to cold storage
        self.cold_storage.insert_solution(solution.clone());

        // Add to hot cache
        let compact = CompactSolution::from_math_solution(
            &solution,
            &mut self.cold_storage.operand_pool,
            self.cold_storage.start_time
        );
        self.hot_cache.put(target_key, compact);

        // Rebuild index if significant growth (every 100 new items)
        if self.cold_storage.solutions.len() % 100 == 0 {
            self.rebuild_index();
        }
    }

    fn rebuild_index(&mut self) {
        self.cold_index = PartitionedIndex::build_from_solutions(
            &self.cold_storage.solutions
        );
    }

    pub fn save(&self) -> Result<()> {
        self.cold_storage.save_to_disk()
    }

    pub fn print_stats(&self) {
        let total = self.hot_hits + self.warm_hits + self.cold_hits + self.misses;
        if total == 0 {
            println!("\n=== Cache Performance ===");
            println!("No queries yet");
            return;
        }

        println!("\n=== Cache Performance ===");
        println!("Hot hits:   {} ({:.1}%)", self.hot_hits,
                 (self.hot_hits as f64 / total as f64) * 100.0);
        println!("Warm hits:  {} ({:.1}%)", self.warm_hits,
                 (self.warm_hits as f64 / total as f64) * 100.0);
        println!("Cold hits:  {} ({:.1}%)", self.cold_hits,
                 (self.cold_hits as f64 / total as f64) * 100.0);
        println!("Misses:     {} ({:.1}%)", self.misses,
                 (self.misses as f64 / total as f64) * 100.0);
        println!("Total solutions: {}", self.cold_storage.len());
    }

    pub fn total_queries(&self) -> u64 {
        self.hot_hits + self.warm_hits + self.cold_hits + self.misses
    }

    pub fn len(&self) -> usize {
        self.cold_storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cold_storage.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiered_memory_basic() {
        let cache = BinaryCache::new("test.bin").unwrap();
        let mut tiered = TieredMemory::new(cache);

        let solution = MathSolution {
            result: 42.0,
            equation: "6 * 7".to_string(),
            accuracy: 100.0,
            timestamp: 0,
            attempts: 1,
        };

        tiered.insert_solution(solution);

        // Should be in hot cache
        let retrieved = tiered.get_solution(42.0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().result, 42.0);
        assert_eq!(tiered.hot_hits, 1);
    }

    #[test]
    fn test_cache_promotion() {
        let cache = BinaryCache::new("test.bin").unwrap();
        let mut tiered = TieredMemory::new(cache);

        // Insert many solutions to fill hot cache
        for i in 0..150 {
            let solution = MathSolution {
                result: i as f64,
                equation: format!("{}", i),
                accuracy: 100.0,
                timestamp: 0,
                attempts: 1,
            };
            tiered.insert_solution(solution);
        }

        // Access an old solution (should be in warm or cold)
        let retrieved = tiered.get_solution(10.0);
        assert!(retrieved.is_some());

        tiered.print_stats();
    }
}
