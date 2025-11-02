use std::f64;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Operation {
    pub result: f64,
    pub equation: String,
    pub formula: String,
}

pub struct EquationSolver {

}

impl EquationSolver {
    pub fn new() -> Self {
        Self {}
    }

    /// Generate all operations with optional formula substitution
    /// formula_map: maps result values to their cumulative formulas
    pub fn generate_all_operations(&self, inputs: &[f64]) -> Vec<Operation> {
        self.generate_all_operations_with_formulas(inputs, &HashMap::new())
    }

    /// Helper to get formula for a value (looks up in formula_map or returns the value as string)
    fn get_formula(&self, value: f64, formula_map: &HashMap<String, String>) -> String {
        let key = format!("{:.10}", value); // Use consistent precision for lookup
        formula_map.get(&key).cloned().unwrap_or_else(|| value.to_string())
    }

    /// Helper to wrap formula in parentheses if needed for operator precedence
    fn wrap_if_needed(&self, formula: &str) -> String {
        // If formula contains operators at the top level, wrap it
        if formula.contains('+') || formula.contains('-') || formula.contains('*') || formula.contains('/') {
            // But not if it's already wrapped or is a function call
            if !formula.starts_with('(') && !formula.contains("avg(") && !formula.contains("sqrt(") {
                return format!("({})", formula);
            }
        }
        formula.to_string()
    }

    pub fn generate_all_operations_with_formulas(&self, inputs: &[f64], formula_map: &HashMap<String, String>) -> Vec<Operation> {
        let mut operations = Vec::new();

        let nums: Vec<f64> = inputs.iter()
            .filter(|&&x| x.is_finite() && !x.is_nan())
            .copied()
            .collect();

        if nums.is_empty() {
            return operations;
        }

        for &num in &nums {
            let formula = self.get_formula(num, formula_map);
            operations.push(Operation {
                result: num,
                equation: num.to_string(),
                formula: formula.clone(),
            });

            // Square root for positive numbers
            if num >= 0.0 {
                operations.push(Operation {
                    result: num.sqrt(),
                    equation: format!("sqrt({})", num),
                    formula: format!("sqrt({})", formula),
                });
            }

            // Absolute value
            operations.push(Operation {
                result: num.abs(),
                equation: format!("abs({})", num),
                formula: format!("abs({})", formula),
            });

            // Square
            operations.push(Operation {
                result: num * num,
                equation: format!("{} ^ 2", num),
                formula: format!("{} ^ 2", formula),
            });

            // Cube
            operations.push(Operation {
                result: num * num * num,
                equation: format!("{} ^ 3", num),
                formula: format!("{} ^ 3", formula),
            });

            // Factorial for small positive integers
            if num >= 0.0 && num <= 12.0 && num.fract() == 0.0 {
                let factorial = self.factorial(num as u32);
                operations.push(Operation {
                    result: factorial,
                    equation: format!("{}!", num),
                    formula: format!("{}!", formula),
                });
            }

            // Ceiling and floor
            operations.push(Operation {
                result: num.ceil(),
                equation: format!("ceil({})", num),
                formula: format!("ceil({})", formula),
            });

            operations.push(Operation {
                result: num.floor(),
                equation: format!("floor({})", num),
                formula: format!("floor({})", formula),
            });
        }
        
        operations.extend(self.generate_two_number_operations(&nums, formula_map));

        if nums.len() >= 3 {
            operations.extend(self.generate_three_number_operations(&nums, formula_map));
        }
        
        operations.into_iter()
            .filter(|op| op.result.is_finite() && !op.result.is_nan())
            .collect()
    }
    
    fn generate_two_number_operations(&self, nums: &[f64], formula_map: &HashMap<String, String>) -> Vec<Operation> {
        // Generate pairs of indices in parallel
        let pairs: Vec<(usize, usize)> = (0..nums.len())
            .flat_map(|i| ((i + 1)..nums.len()).map(move |j| (i, j)))
            .collect();

        pairs.par_iter()
            .flat_map(|&(i, j)| {
                let a = nums[i];
                let b = nums[j];
                let a_formula = self.get_formula(a, formula_map);
                let b_formula = self.get_formula(b, formula_map);
                let mut ops = Vec::new();

                ops.push(Operation {
                    result: a + b,
                    equation: format!("{} + {}", a, b),
                    formula: format!("{} + {}", a_formula, b_formula),
                });

                ops.push(Operation {
                    result: a - b,
                    equation: format!("{} - {}", a, b),
                    formula: format!("{} - {}", a_formula, b_formula),
                });

                ops.push(Operation {
                    result: b - a,
                    equation: format!("{} - {}", b, a),
                    formula: format!("{} - {}", b_formula, a_formula),
                });

                ops.push(Operation {
                    result: a * b,
                    equation: format!("{} * {}", a, b),
                    formula: format!("{} * {}", a_formula, b_formula),
                });

                if b.abs() > f64::EPSILON {
                    ops.push(Operation {
                        result: a / b,
                        equation: format!("{} / {}", a, b),
                        formula: format!("{} / {}", a_formula, b_formula),
                    });
                }

                if a.abs() > f64::EPSILON {
                    ops.push(Operation {
                        result: b / a,
                        equation: format!("{} / {}", b, a),
                        formula: format!("{} / {}", b_formula, a_formula),
                    });
                }

                if a.abs() <= 100.0 && b.abs() <= 10.0 && b >= 0.0 {
                    let pow_result = a.powf(b);
                    if pow_result.is_finite() && !pow_result.is_nan() {
                        ops.push(Operation {
                            result: pow_result,
                            equation: format!("{} ^ {}", a, b),
                            formula: format!("{} ^ {}", a_formula, b_formula),
                        });
                    }
                }

                if b.abs() <= 100.0 && a.abs() <= 10.0 && a >= 0.0 {
                    let pow_result = b.powf(a);
                    if pow_result.is_finite() && !pow_result.is_nan() {
                        ops.push(Operation {
                            result: pow_result,
                            equation: format!("{} ^ {}", b, a),
                            formula: format!("{} ^ {}", b_formula, a_formula),
                        });
                    }
                }

                if b.abs() > f64::EPSILON {
                    ops.push(Operation {
                        result: a % b,
                        equation: format!("{} % {}", a, b),
                        formula: format!("{} % {}", a_formula, b_formula),
                    });
                }

                if a.abs() > f64::EPSILON {
                    ops.push(Operation {
                        result: b % a,
                        equation: format!("{} % {}", b, a),
                        formula: format!("{} % {}", b_formula, a_formula),
                    });
                }

                ops.push(Operation {
                    result: a.max(b),
                    equation: format!("max({}, {})", a, b),
                    formula: format!("max({}, {})", a_formula, b_formula),
                });

                ops.push(Operation {
                    result: a.min(b),
                    equation: format!("min({}, {})", a, b),
                    formula: format!("min({}, {})", a_formula, b_formula),
                });

                ops.push(Operation {
                    result: a.hypot(b),
                    equation: format!("hypot({}, {})", a, b),
                    formula: format!("hypot({}, {})", a_formula, b_formula),
                });

                ops.push(Operation {
                    result: a.atan2(b),
                    equation: format!("atan2({}, {})", a, b),
                    formula: format!("atan2({}, {})", a_formula, b_formula),
                });

                // Average
                ops.push(Operation {
                    result: (a + b) / 2.0,
                    equation: format!("avg({}, {})", a, b),
                    formula: format!("avg({}, {})", a_formula, b_formula),
                });

                // Geometric mean for positive numbers
                if a > 0.0 && b > 0.0 {
                    ops.push(Operation {
                        result: (a * b).sqrt(),
                        equation: format!("geomean({}, {})", a, b),
                        formula: format!("geomean({}, {})", a_formula, b_formula),
                    });
                }

                ops
            })
            .collect()
    }

    /// Calculate factorial for small numbers (up to 12! = 479,001,600)
    fn factorial(&self, n: u32) -> f64 {
        match n {
            0 | 1 => 1.0,
            2 => 2.0,
            3 => 6.0,
            4 => 24.0,
            5 => 120.0,
            6 => 720.0,
            7 => 5040.0,
            8 => 40320.0,
            9 => 362880.0,
            10 => 3628800.0,
            11 => 39916800.0,
            12 => 479001600.0,
            _ => {
                // For larger numbers, compute iteratively
                let mut result = 1.0;
                for i in 2..=n {
                    result *= i as f64;
                }
                result
            }
        }
    }
    
    fn generate_three_number_operations(&self, nums: &[f64], formula_map: &HashMap<String, String>) -> Vec<Operation> {
        // Generate triplets of indices in parallel
        let triplets: Vec<(usize, usize, usize)> = (0..nums.len())
            .flat_map(|i| {
                ((i + 1)..nums.len()).flat_map(move |j| {
                    ((j + 1)..nums.len()).map(move |k| (i, j, k))
                })
            })
            .collect();

        triplets.par_iter()
            .flat_map(|&(i, j, k)| {
                let a = nums[i];
                let b = nums[j];
                let c = nums[k];
                let a_formula = self.get_formula(a, formula_map);
                let b_formula = self.get_formula(b, formula_map);
                let c_formula = self.get_formula(c, formula_map);
                let mut ops = Vec::new();

                ops.push(Operation {
                    result: a + b + c,
                    equation: format!("{} + {} + {}", a, b, c),
                    formula: format!("{} + {} + {}", a_formula, b_formula, c_formula),
                });

                ops.push(Operation {
                        result: a + b - c,
                        equation: format!("{} + {} - {}", a, b, c),
                        formula: format!("{} + {} - {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a - b + c,
                        equation: format!("{} - {} + {}", a, b, c),
                        formula: format!("{} - {} + {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a - b - c,
                        equation: format!("{} - {} - {}", a, b, c),
                        formula: format!("{} - {} - {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a * b + c,
                        equation: format!("{} * {} + {}", a, b, c),
                        formula: format!("{} * {} + {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a * b - c,
                        equation: format!("{} * {} - {}", a, b, c),
                        formula: format!("{} * {} - {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a + b * c,
                        equation: format!("{} + {} * {}", a, b, c),
                        formula: format!("{} + {} * {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a - b * c,
                        equation: format!("{} - {} * {}", a, b, c),
                        formula: format!("{} - {} * {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a * c + b,
                        equation: format!("{} * {} + {}", a, c, b),
                        formula: format!("{} * {} + {}", a_formula, c_formula, b_formula),
                    });

                ops.push(Operation {
                        result: a * c - b,
                        equation: format!("{} * {} - {}", a, c, b),
                        formula: format!("{} * {} - {}", a_formula, c_formula, b_formula),
                    });

                ops.push(Operation {
                        result: b * c + a,
                        equation: format!("{} * {} + {}", b, c, a),
                        formula: format!("{} * {} + {}", b_formula, c_formula, a_formula),
                    });

                ops.push(Operation {
                        result: b * c - a,
                        equation: format!("{} * {} - {}", b, c, a),
                        formula: format!("{} * {} - {}", b_formula, c_formula, a_formula),
                    });
                    
                ops.push(Operation {
                        result: (a + b) * c,
                        equation: format!("({} + {}) * {}", a, b, c),
                        formula: format!("({} + {}) * {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: (a - b) * c,
                        equation: format!("({} - {}) * {}", a, b, c),
                        formula: format!("({} - {}) * {}", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a * (b + c),
                        equation: format!("{} * ({} + {})", a, b, c),
                        formula: format!("{} * ({} + {})", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: a * (b - c),
                        equation: format!("{} * ({} - {})", a, b, c),
                        formula: format!("{} * ({} - {})", a_formula, b_formula, c_formula),
                    });

                ops.push(Operation {
                        result: (a + c) * b,
                        equation: format!("({} + {}) * {}", a, c, b),
                        formula: format!("({} + {}) * {}", a_formula, c_formula, b_formula),
                    });

                ops.push(Operation {
                        result: (a - c) * b,
                        equation: format!("({} - {}) * {}", a, c, b),
                        formula: format!("({} - {}) * {}", a_formula, c_formula, b_formula),
                    });

                ops.push(Operation {
                        result: b * (a + c),
                        equation: format!("{} * ({} + {})", b, a, c),
                        formula: format!("{} * ({} + {})", b_formula, a_formula, c_formula),
                    });

                ops.push(Operation {
                        result: b * (a - c),
                        equation: format!("{} * ({} - {})", b, a, c),
                        formula: format!("{} * ({} - {})", b_formula, a_formula, c_formula),
                    });
                    
                    if c.abs() > f64::EPSILON {
                    ops.push(Operation {
                            result: (a + b) / c,
                            equation: format!("({} + {}) / {}", a, b, c),
                            formula: format!("({} + {}) / {}", a_formula, b_formula, c_formula),
                        });

                    ops.push(Operation {
                            result: (a - b) / c,
                            equation: format!("({} - {}) / {}", a, b, c),
                            formula: format!("({} - {}) / {}", a_formula, b_formula, c_formula),
                        });

                    ops.push(Operation {
                            result: (a * b) / c,
                            equation: format!("({} * {}) / {}", a, b, c),
                            formula: format!("({} * {}) / {}", a_formula, b_formula, c_formula),
                        });
                    }

                    if b.abs() > f64::EPSILON {
                    ops.push(Operation {
                            result: (a + c) / b,
                            equation: format!("({} + {}) / {}", a, c, b),
                            formula: format!("({} + {}) / {}", a_formula, c_formula, b_formula),
                        });

                    ops.push(Operation {
                            result: (a - c) / b,
                            equation: format!("({} - {}) / {}", a, c, b),
                            formula: format!("({} - {}) / {}", a_formula, c_formula, b_formula),
                        });

                    ops.push(Operation {
                            result: (a * c) / b,
                            equation: format!("({} * {}) / {}", a, c, b),
                            formula: format!("({} * {}) / {}", a_formula, c_formula, b_formula),
                        });
                    }

                    if a.abs() > f64::EPSILON {
                    ops.push(Operation {
                            result: (b + c) / a,
                            equation: format!("({} + {}) / {}", b, c, a),
                            formula: format!("({} + {}) / {}", b_formula, c_formula, a_formula),
                        });

                    ops.push(Operation {
                            result: (b - c) / a,
                            equation: format!("({} - {}) / {}", b, c, a),
                            formula: format!("({} - {}) / {}", b_formula, c_formula, a_formula),
                        });

                    ops.push(Operation {
                            result: (b * c) / a,
                            equation: format!("({} * {}) / {}", b, c, a),
                            formula: format!("({} * {}) / {}", b_formula, c_formula, a_formula),
                        });
                    }

                    if b.abs() > f64::EPSILON && c.abs() > f64::EPSILON {
                    ops.push(Operation {
                            result: a / b / c,
                            equation: format!("{} / {} / {}", a, b, c),
                            formula: format!("{} / {} / {}", a_formula, b_formula, c_formula),
                        });
                    }

                    if a.abs() > f64::EPSILON && c.abs() > f64::EPSILON {
                    ops.push(Operation {
                            result: b / a / c,
                            equation: format!("{} / {} / {}", b, a, c),
                            formula: format!("{} / {} / {}", b_formula, a_formula, c_formula),
                        });
                    }

                    if a.abs() > f64::EPSILON && b.abs() > f64::EPSILON {
                    ops.push(Operation {
                            result: c / a / b,
                            equation: format!("{} / {} / {}", c, a, b),
                            formula: format!("{} / {} / {}", c_formula, a_formula, b_formula),
                        });
                    }

                ops.push(Operation {
                        result: a * b * c,
                        equation: format!("{} * {} * {}", a, b, c),
                        formula: format!("{} * {} * {}", a_formula, b_formula, c_formula),
                    });
                    
                    if a.abs() <= 10.0 && b.abs() <= 5.0 && b >= 0.0 {
                        let pow_result = a.powf(b) + c;
                        if pow_result.is_finite() && !pow_result.is_nan() {
                        ops.push(Operation {
                                result: pow_result,
                                equation: format!("{} ^ {} + {}", a, b, c),
                                formula: format!("{} ^ {} + {}", a_formula, b_formula, c_formula),
                            });
                        }

                        let pow_result = a.powf(b) - c;
                        if pow_result.is_finite() && !pow_result.is_nan() {
                        ops.push(Operation {
                                result: pow_result,
                                equation: format!("{} ^ {} - {}", a, b, c),
                                formula: format!("{} ^ {} - {}", a_formula, b_formula, c_formula),
                            });
                        }
                    }

                    if (a + b).abs() <= 10.0 && c.abs() <= 5.0 && c >= 0.0 {
                        let pow_result = (a + b).powf(c);
                        if pow_result.is_finite() && !pow_result.is_nan() {
                        ops.push(Operation {
                                result: pow_result,
                                equation: format!("({} + {}) ^ {}", a, b, c),
                                formula: format!("({} + {}) ^ {}", a_formula, b_formula, c_formula),
                            });
                        }
                    }

                    if (a - b).abs() <= 10.0 && c.abs() <= 5.0 && c >= 0.0 {
                        let pow_result = (a - b).powf(c);
                        if pow_result.is_finite() && !pow_result.is_nan() {
                        ops.push(Operation {
                                result: pow_result,
                                equation: format!("({} - {}) ^ {}", a, b, c),
                                formula: format!("({} - {}) ^ {}", a_formula, b_formula, c_formula),
                            });
                        }
                    }

                // Average of three numbers
                ops.push(Operation {
                    result: (a + b + c) / 3.0,
                    equation: format!("avg({}, {}, {})", a, b, c),
                    formula: format!("avg({}, {}, {})", a_formula, b_formula, c_formula),
                });

                // Geometric mean for three positive numbers
                if a > 0.0 && b > 0.0 && c > 0.0 {
                    ops.push(Operation {
                        result: (a * b * c).cbrt(),
                        equation: format!("geomean({}, {}, {})", a, b, c),
                        formula: format!("geomean({}, {}, {})", a_formula, b_formula, c_formula),
                    });
                }

                ops
            })
            .collect()
    }
}