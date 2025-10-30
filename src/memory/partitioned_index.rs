use std::collections::BTreeMap;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};

use super::compact_solution::CompactSolution;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionedIndex {
    // Three partitions for parallel search
    head: BTreeMap<u32, usize>,    // First 33%
    middle: BTreeMap<u32, usize>,  // Middle 33%
    tail: BTreeMap<u32, usize>,    // Last 33%

    // Partition boundaries
    head_max: f32,
    middle_max: f32,
}

impl PartitionedIndex {
    pub fn build_from_solutions(solutions: &[CompactSolution]) -> Self {
        if solutions.is_empty() {
            return Self::default();
        }

        // Sort to find boundaries
        let mut results: Vec<f32> = solutions.iter()
            .map(|s| s.result)
            .collect();
        results.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let third = results.len() / 3;
        let head_max = if third > 0 { results[third] } else { results[0] };
        let middle_max = if third * 2 < results.len() {
            results[third * 2]
        } else {
            results[results.len() - 1]
        };

        // Build indices
        let mut head = BTreeMap::new();
        let mut middle = BTreeMap::new();
        let mut tail = BTreeMap::new();

        for (i, solution) in solutions.iter().enumerate() {
            let key = (solution.result * 100.0) as u32;

            if solution.result <= head_max {
                head.insert(key, i);
            } else if solution.result <= middle_max {
                middle.insert(key, i);
            } else {
                tail.insert(key, i);
            }
        }

        println!(">> Built partitioned index:");
        println!("   Head: {} entries (0.0 - {})", head.len(), head_max);
        println!("   Middle: {} entries ({} - {})", middle.len(), head_max, middle_max);
        println!("   Tail: {} entries ({} - max)", tail.len(), middle_max);

        Self {
            head,
            middle,
            tail,
            head_max,
            middle_max,
        }
    }

    pub fn parallel_search(&self, target: f32) -> Option<usize> {
        let target_key = (target * 100.0) as u32;

        // Search all three partitions in parallel
        let results: Vec<Option<usize>> = vec![
            &self.head,
            &self.middle,
            &self.tail,
        ]
        .par_iter()
        .map(|partition| partition.get(&target_key).copied())
        .collect();

        // Return first match found
        results.into_iter().find_map(|r| r)
    }

    pub fn smart_search(&self, target: f32) -> Option<usize> {
        let target_key = (target * 100.0) as u32;

        // Route to correct partition (single-threaded optimization)
        if target <= self.head_max {
            self.head.get(&target_key).copied()
        } else if target <= self.middle_max {
            self.middle.get(&target_key).copied()
        } else {
            self.tail.get(&target_key).copied()
        }
    }

    pub fn len(&self) -> usize {
        self.head.len() + self.middle.len() + self.tail.len()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_empty() && self.middle.is_empty() && self.tail.is_empty()
    }
}

impl Default for PartitionedIndex {
    fn default() -> Self {
        Self {
            head: BTreeMap::new(),
            middle: BTreeMap::new(),
            tail: BTreeMap::new(),
            head_max: 0.0,
            middle_max: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::compact_solution::{CompactSolution, OperandPool};
    use crate::MathSolution;

    #[test]
    fn test_partitioned_index_basic() {
        let mut pool = OperandPool::new();
        let start_time = 0;

        let mut solutions = Vec::new();
        for i in 0..30 {
            let sol = MathSolution {
                result: i as f64,
                equation: format!("{}", i),
                accuracy: 100.0,
                timestamp: start_time,
                attempts: 1,
            };
            solutions.push(CompactSolution::from_math_solution(&sol, &mut pool, start_time));
        }

        let index = PartitionedIndex::build_from_solutions(&solutions);

        // Test smart search
        assert!(index.smart_search(5.0).is_some());
        assert!(index.smart_search(15.0).is_some());
        assert!(index.smart_search(25.0).is_some());
    }
}
