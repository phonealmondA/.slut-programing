use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use ordered_float::OrderedFloat;
use regex::Regex;

// Import MathSolution from parent module
use crate::MathSolution;

/// Compact binary representation of a solution
/// Size: 13 bytes (vs 50-200 bytes JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(packed)]
pub struct CompactSolution {
    /// Result value (f32 sufficient for most cases)
    pub result: f32,              // 4 bytes

    /// Operation code (see OPERATION_CODES table)
    pub operation_code: u8,       // 1 byte

    /// Operand indices (references to operand pool)
    pub operands: [u16; 3],       // 6 bytes (3 × 2 bytes)

    /// Timestamp delta (seconds since program start)
    pub timestamp_delta: u16,     // 2 bytes
}

impl CompactSolution {
    /// Convert from old MathSolution format
    pub fn from_math_solution(
        solution: &MathSolution,
        operand_pool: &mut OperandPool,
        start_time: u64
    ) -> Self {
        let operation_code = parse_operation_code(&solution.equation);
        let operands = operand_pool.register_operands(&solution.equation);
        let timestamp_delta = solution.timestamp.saturating_sub(start_time)
            .checked_div(1000)
            .unwrap_or(0)
            .min(u16::MAX as u64) as u16;

        CompactSolution {
            result: solution.result as f32,
            operation_code,
            operands,
            timestamp_delta,
        }
    }

    /// Convert back to MathSolution for compatibility
    pub fn to_math_solution(
        &self,
        operand_pool: &OperandPool,
        start_time: u64
    ) -> MathSolution {
        // Copy operands to avoid packed field alignment issues
        let operands_copy = self.operands;
        let equation = reconstruct_equation(
            self.operation_code,
            &operands_copy,
            operand_pool
        );

        MathSolution {
            result: self.result as f64,
            equation,
            accuracy: 100.0,  // Cached solutions are always accurate
            timestamp: start_time + (self.timestamp_delta as u64 * 1000),
            attempts: 1,
        }
    }
}

/// Pool of unique operands (numbers used in equations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperandPool {
    operands: Vec<f32>,
    lookup: HashMap<OrderedFloat<f32>, u16>,
}

impl OperandPool {
    pub fn new() -> Self {
        Self {
            operands: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn register(&mut self, value: f32) -> u16 {
        let key = OrderedFloat(value);
        if let Some(&index) = self.lookup.get(&key) {
            return index;
        }

        let index = self.operands.len() as u16;
        self.operands.push(value);
        self.lookup.insert(key, index);
        index
    }

    pub fn get(&self, index: u16) -> Option<f32> {
        self.operands.get(index as usize).copied()
    }

    pub fn register_operands(&mut self, equation: &str) -> [u16; 3] {
        let numbers = extract_numbers_from_equation(equation);
        let mut operands = [0u16; 3];

        for (i, num) in numbers.iter().take(3).enumerate() {
            operands[i] = self.register(*num);
        }

        operands
    }
}

impl Default for OperandPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Operation code table (256 possible operations)
pub const OPERATION_CODES: &[&str] = &[
    "a + b",              // 0x00
    "a - b",              // 0x01
    "a * b",              // 0x02
    "a / b",              // 0x03
    "a ^ b",              // 0x04
    "a + b + c",          // 0x05
    "a * b * c",          // 0x06
    "(a + b) * c",        // 0x07
    "(a - b) * c",        // 0x08
    "a * (b + c)",        // 0x09
    "a * (b - c)",        // 0x0A
    "(a + b) / c",        // 0x0B
    "a ^ b + c",          // 0x0C
    "a ^ b - c",          // 0x0D
    "(a + b) ^ c",        // 0x0E
    "a * b + c",          // 0x0F
    "a * b - c",          // 0x10
    "a / b + c",          // 0x11
    "a / b - c",          // 0x12
    "a + b * c",          // 0x13
    "a - b * c",          // 0x14
    "a + b / c",          // 0x15
    "a - b / c",          // 0x16
];

fn parse_operation_code(equation: &str) -> u8 {
    // Simplified pattern matching based on operators
    let add_count = equation.matches('+').count();
    let sub_count = equation.matches('-').count();
    let mul_count = equation.matches('*').count();
    let div_count = equation.matches('/').count();
    let pow_count = equation.matches('^').count();
    let paren_count = equation.matches('(').count();

    // Match based on operation signature
    match (add_count, sub_count, mul_count, div_count, pow_count, paren_count) {
        (1, 0, 0, 0, 0, 0) => 0,  // a + b
        (0, 1, 0, 0, 0, 0) => 1,  // a - b
        (0, 0, 1, 0, 0, 0) => 2,  // a * b
        (0, 0, 0, 1, 0, 0) => 3,  // a / b
        (0, 0, 0, 0, 1, 0) => 4,  // a ^ b
        (2, 0, 0, 0, 0, 0) => 5,  // a + b + c
        (0, 0, 2, 0, 0, 0) => 6,  // a * b * c
        (1, 0, 1, 0, 0, 1) => 7,  // (a + b) * c
        (0, 1, 1, 0, 0, 1) => 8,  // (a - b) * c
        (1, 0, 1, 0, 0, 0) => 15, // a * b + c or a + b * c
        (0, 1, 1, 0, 0, 0) => 16, // a * b - c or a - b * c
        (1, 0, 0, 1, 0, 0) => 17, // a / b + c or a + b / c
        (0, 1, 0, 1, 0, 0) => 18, // a / b - c or a - b / c
        _ => 0, // Default to a + b
    }
}

fn reconstruct_equation(
    op_code: u8,
    operands: &[u16; 3],
    pool: &OperandPool
) -> String {
    let pattern = OPERATION_CODES.get(op_code as usize)
        .unwrap_or(&"a + b");

    let a = pool.get(operands[0]).unwrap_or(0.0);
    let b = pool.get(operands[1]).unwrap_or(0.0);
    let c = pool.get(operands[2]).unwrap_or(0.0);

    pattern
        .replace("a", &format!("{}", a))
        .replace("b", &format!("{}", b))
        .replace("c", &format!("{}", c))
}

fn extract_numbers_from_equation(equation: &str) -> Vec<f32> {
    // Simple regex to extract numbers (including decimals and negatives)
    let re = Regex::new(r"-?\d+\.?\d*").unwrap();

    re.find_iter(equation)
        .filter_map(|m| m.as_str().parse::<f32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_solution_size() {
        use std::mem::size_of;
        assert!(size_of::<CompactSolution>() <= 16, "CompactSolution should be compact");
    }

    #[test]
    fn test_operand_pool() {
        let mut pool = OperandPool::new();
        let idx1 = pool.register(42.0);
        let idx2 = pool.register(3.14);
        let idx3 = pool.register(42.0); // Duplicate

        assert_eq!(idx1, idx3); // Should return same index for duplicate
        assert_ne!(idx1, idx2);
        assert_eq!(pool.get(idx1), Some(42.0));
        assert_eq!(pool.get(idx2), Some(3.14));
    }

    #[test]
    fn test_extract_numbers() {
        let nums = extract_numbers_from_equation("2 + 3.14 * 5");
        assert_eq!(nums, vec![2.0, 3.14, 5.0]);
    }
}
