use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use bincode;
use anyhow::{Result, Context};

use super::compact_solution::{CompactSolution, OperandPool};
use crate::MathSolution;

pub struct BinaryCache {
    pub solutions: Vec<CompactSolution>,
    pub operand_pool: OperandPool,
    pub start_time: u64,
    file_path: String,
}

impl BinaryCache {
    pub fn new(file_path: &str) -> Result<Self> {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;

        Ok(Self {
            solutions: Vec::new(),
            operand_pool: OperandPool::new(),
            start_time,
            file_path: file_path.to_string(),
        })
    }

    pub fn from_hashmap(
        math_solutions: std::collections::HashMap<String, MathSolution>
    ) -> Result<Self> {
        let mut binary_cache = Self::new("quantum_cache.bin")?;

        println!(">> Converting {} solutions to binary format...", math_solutions.len());
        let start = std::time::Instant::now();

        for (_key, solution) in &math_solutions {
            let compact = CompactSolution::from_math_solution(
                solution,
                &mut binary_cache.operand_pool,
                binary_cache.start_time
            );
            binary_cache.solutions.push(compact);
        }

        let duration = start.elapsed();
        println!("   Converted {} solutions in {:?}",
                 binary_cache.solutions.len(), duration);

        Ok(binary_cache)
    }

    pub fn save_to_disk(&self) -> Result<()> {
        let start = std::time::Instant::now();

        let encoded = bincode::serialize(&(
            &self.solutions,
            &self.operand_pool,
            self.start_time
        ))
        .context("Failed to serialize binary cache")?;

        let mut file = File::create(&self.file_path)
            .context("Failed to create binary cache file")?;
        file.write_all(&encoded)
            .context("Failed to write binary cache")?;

        let duration = start.elapsed();
        let size_kb = encoded.len() / 1024;

        println!(">> Saved binary cache: {} KB in {:?}", size_kb, duration);

        Ok(())
    }

    pub fn load_from_disk(file_path: &str) -> Result<Self> {
        if !Path::new(file_path).exists() {
            return Err(anyhow::anyhow!("Binary cache file not found: {}", file_path));
        }

        let start = std::time::Instant::now();

        let mut file = File::open(file_path)
            .context("Failed to open binary cache file")?;
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded)
            .context("Failed to read binary cache")?;

        let (solutions, operand_pool, start_time): (Vec<CompactSolution>, OperandPool, u64) =
            bincode::deserialize(&encoded)
                .context("Failed to deserialize binary cache")?;

        let duration = start.elapsed();
        println!(">> Loaded {} solutions from binary cache in {:?}",
                 solutions.len(), duration);

        Ok(Self {
            solutions,
            operand_pool,
            start_time,
            file_path: file_path.to_string(),
        })
    }

    pub fn get_solution(&self, target: f32) -> Option<MathSolution> {
        self.solutions.iter()
            .find(|s| (s.result - target).abs() < 0.01)
            .map(|compact| compact.to_math_solution(&self.operand_pool, self.start_time))
    }

    pub fn insert_solution(&mut self, solution: MathSolution) {
        let compact = CompactSolution::from_math_solution(
            &solution,
            &mut self.operand_pool,
            self.start_time
        );
        self.solutions.push(compact);
    }

    pub fn len(&self) -> usize {
        self.solutions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.solutions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_cache_creation() {
        let cache = BinaryCache::new("test.bin").unwrap();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_insert_and_get() {
        let mut cache = BinaryCache::new("test.bin").unwrap();

        let solution = MathSolution {
            result: 42.0,
            equation: "2 * 3 * 7".to_string(),
            accuracy: 100.0,
            timestamp: cache.start_time,
            attempts: 1,
        };

        cache.insert_solution(solution);
        assert_eq!(cache.len(), 1);

        let retrieved = cache.get_solution(42.0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().result, 42.0);
    }
}
