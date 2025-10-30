// Memory optimization module
// Implements multi-tier caching system with binary storage

pub mod compact_solution;
pub mod binary_cache;
pub mod bloom_filter;
pub mod partitioned_index;
pub mod tiered_cache;

// Re-export main types for convenience
pub use compact_solution::{CompactSolution, OperandPool};
pub use binary_cache::BinaryCache;
pub use bloom_filter::BloomFilter;
pub use partitioned_index::PartitionedIndex;
pub use tiered_cache::TieredMemory;
