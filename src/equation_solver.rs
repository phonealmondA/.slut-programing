use std::f64;

#[derive(Debug, Clone)]
pub struct Operation {
    pub result: f64,
    pub equation: String,
}

pub struct EquationSolver {
    // Could store configuration or optimization data in the future
}

impl EquationSolver {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn generate_all_operations(&self, inputs: &[f64]) -> Vec<Operation> {
        let mut operations = Vec::new();
        
        // Filter valid numeric inputs
        let nums: Vec<f64> = inputs.iter()
            .filter(|&&x| x.is_finite() && !x.is_nan())
            .copied()
            .collect();
        
        if nums.is_empty() {
            return operations;
        }
        
        // Single number operations (identity)
        for &num in &nums {
            operations.push(Operation {
                result: num,
                equation: num.to_string(),
            });
        }
        
        // Two-number operations
        operations.extend(self.generate_two_number_operations(&nums));
        
        // Three-number operations
        if nums.len() >= 3 {
            operations.extend(self.generate_three_number_operations(&nums));
        }
        
        // Filter out invalid results
        operations.into_iter()
            .filter(|op| op.result.is_finite() && !op.result.is_nan())
            .collect()
    }
    
    fn generate_two_number_operations(&self, nums: &[f64]) -> Vec<Operation> {
        let mut operations = Vec::new();
        
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let a = nums[i];
                let b = nums[j];
                
                // Basic arithmetic
                operations.push(Operation {
                    result: a + b,
                    equation: format!("{} + {}", a, b),
                });
                
                operations.push(Operation {
                    result: a - b,
                    equation: format!("{} - {}", a, b),
                });
                
                operations.push(Operation {
                    result: b - a,
                    equation: format!("{} - {}", b, a),
                });
                
                operations.push(Operation {
                    result: a * b,
                    equation: format!("{} * {}", a, b),
                });
                
                // Division (with zero check)
                if b.abs() > f64::EPSILON {
                    operations.push(Operation {
                        result: a / b,
                        equation: format!("{} / {}", a, b),
                    });
                }
                
                if a.abs() > f64::EPSILON {
                    operations.push(Operation {
                        result: b / a,
                        equation: format!("{} / {}", b, a),
                    });
                }
                
                // Exponentiation (with reasonable limits)
                if a.abs() <= 100.0 && b.abs() <= 10.0 && b >= 0.0 {
                    let pow_result = a.powf(b);
                    if pow_result.is_finite() && !pow_result.is_nan() {
                        operations.push(Operation {
                            result: pow_result,
                            equation: format!("{} ^ {}", a, b),
                        });
                    }
                }
                
                if b.abs() <= 100.0 && a.abs() <= 10.0 && a >= 0.0 {
                    let pow_result = b.powf(a);
                    if pow_result.is_finite() && !pow_result.is_nan() {
                        operations.push(Operation {
                            result: pow_result,
                            equation: format!("{} ^ {}", b, a),
                        });
                    }
                }
                
                // Modulo (with zero check)
                if b.abs() > f64::EPSILON {
                    operations.push(Operation {
                        result: a % b,
                        equation: format!("{} % {}", a, b),
                    });
                }
                
                if a.abs() > f64::EPSILON {
                    operations.push(Operation {
                        result: b % a,
                        equation: format!("{} % {}", b, a),
                    });
                }
                
                // Mathematical functions
                operations.push(Operation {
                    result: a.max(b),
                    equation: format!("max({}, {})", a, b),
                });
                
                operations.push(Operation {
                    result: a.min(b),
                    equation: format!("min({}, {})", a, b),
                });
                
                operations.push(Operation {
                    result: a.hypot(b),
                    equation: format!("hypot({}, {})", a, b),
                });
                
                operations.push(Operation {
                    result: a.atan2(b),
                    equation: format!("atan2({}, {})", a, b),
                });
            }
        }
        
        operations
    }
    
    fn generate_three_number_operations(&self, nums: &[f64]) -> Vec<Operation> {
        let mut operations = Vec::new();
        
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let a = nums[i];
                    let b = nums[j];
                    let c = nums[k];
                    
                    // Three-number addition/subtraction combinations
                    operations.push(Operation {
                        result: a + b + c,
                        equation: format!("{} + {} + {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a + b - c,
                        equation: format!("{} + {} - {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a - b + c,
                        equation: format!("{} - {} + {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a - b - c,
                        equation: format!("{} - {} - {}", a, b, c),
                    });
                    
                    // Mixed multiplication and addition/subtraction
                    operations.push(Operation {
                        result: a * b + c,
                        equation: format!("{} * {} + {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a * b - c,
                        equation: format!("{} * {} - {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a + b * c,
                        equation: format!("{} + {} * {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a - b * c,
                        equation: format!("{} - {} * {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a * c + b,
                        equation: format!("{} * {} + {}", a, c, b),
                    });
                    
                    operations.push(Operation {
                        result: a * c - b,
                        equation: format!("{} * {} - {}", a, c, b),
                    });
                    
                    operations.push(Operation {
                        result: b * c + a,
                        equation: format!("{} * {} + {}", b, c, a),
                    });
                    
                    operations.push(Operation {
                        result: b * c - a,
                        equation: format!("{} * {} - {}", b, c, a),
                    });
                    
                    // Grouped operations
                    operations.push(Operation {
                        result: (a + b) * c,
                        equation: format!("({} + {}) * {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: (a - b) * c,
                        equation: format!("({} - {}) * {}", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a * (b + c),
                        equation: format!("{} * ({} + {})", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: a * (b - c),
                        equation: format!("{} * ({} - {})", a, b, c),
                    });
                    
                    operations.push(Operation {
                        result: (a + c) * b,
                        equation: format!("({} + {}) * {}", a, c, b),
                    });
                    
                    operations.push(Operation {
                        result: (a - c) * b,
                        equation: format!("({} - {}) * {}", a, c, b),
                    });
                    
                    operations.push(Operation {
                        result: b * (a + c),
                        equation: format!("{} * ({} + {})", b, a, c),
                    });
                    
                    operations.push(Operation {
                        result: b * (a - c),
                        equation: format!("{} * ({} - {})", b, a, c),
                    });
                    
                    // Division operations (with zero checks)
                    if c.abs() > f64::EPSILON {
                        operations.push(Operation {
                            result: (a + b) / c,
                            equation: format!("({} + {}) / {}", a, b, c),
                        });
                        
                        operations.push(Operation {
                            result: (a - b) / c,
                            equation: format!("({} - {}) / {}", a, b, c),
                        });
                        
                        operations.push(Operation {
                            result: (a * b) / c,
                            equation: format!("({} * {}) / {}", a, b, c),
                        });
                    }
                    
                    if b.abs() > f64::EPSILON {
                        operations.push(Operation {
                            result: (a + c) / b,
                            equation: format!("({} + {}) / {}", a, c, b),
                        });
                        
                        operations.push(Operation {
                            result: (a - c) / b,
                            equation: format!("({} - {}) / {}", a, c, b),
                        });
                        
                        operations.push(Operation {
                            result: (a * c) / b,
                            equation: format!("({} * {}) / {}", a, c, b),
                        });
                    }
                    
                    if a.abs() > f64::EPSILON {
                        operations.push(Operation {
                            result: (b + c) / a,
                            equation: format!("({} + {}) / {}", b, c, a),
                        });
                        
                        operations.push(Operation {
                            result: (b - c) / a,
                            equation: format!("({} - {}) / {}", b, c, a),
                        });
                        
                        operations.push(Operation {
                            result: (b * c) / a,
                            equation: format!("({} * {}) / {}", b, c, a),
                        });
                    }
                    
                    // Chain division (with zero checks)
                    if b.abs() > f64::EPSILON && c.abs() > f64::EPSILON {
                        operations.push(Operation {
                            result: a / b / c,
                            equation: format!("{} / {} / {}", a, b, c),
                        });
                    }
                    
                    if a.abs() > f64::EPSILON && c.abs() > f64::EPSILON {
                        operations.push(Operation {
                            result: b / a / c,
                            equation: format!("{} / {} / {}", b, a, c),
                        });
                    }
                    
                    if a.abs() > f64::EPSILON && b.abs() > f64::EPSILON {
                        operations.push(Operation {
                            result: c / a / b,
                            equation: format!("{} / {} / {}", c, a, b),
                        });
                    }
                    
                    // Three-number multiplication
                    operations.push(Operation {
                        result: a * b * c,
                        equation: format!("{} * {} * {}", a, b, c),
                    });
                    
                    // Exponentiation with third number (with limits)
                    if a.abs() <= 10.0 && b.abs() <= 5.0 && b >= 0.0 {
                        let pow_result = a.powf(b) + c;
                        if pow_result.is_finite() && !pow_result.is_nan() {
                            operations.push(Operation {
                                result: pow_result,
                                equation: format!("{} ^ {} + {}", a, b, c),
                            });
                        }
                        
                        let pow_result = a.powf(b) - c;
                        if pow_result.is_finite() && !pow_result.is_nan() {
                            operations.push(Operation {
                                result: pow_result,
                                equation: format!("{} ^ {} - {}", a, b, c),
                            });
                        }
                    }
                    
                    if (a + b).abs() <= 10.0 && c.abs() <= 5.0 && c >= 0.0 {
                        let pow_result = (a + b).powf(c);
                        if pow_result.is_finite() && !pow_result.is_nan() {
                            operations.push(Operation {
                                result: pow_result,
                                equation: format!("({} + {}) ^ {}", a, b, c),
                            });
                        }
                    }
                    
                    if (a - b).abs() <= 10.0 && c.abs() <= 5.0 && c >= 0.0 {
                        let pow_result = (a - b).powf(c);
                        if pow_result.is_finite() && !pow_result.is_nan() {
                            operations.push(Operation {
                                result: pow_result,
                                equation: format!("({} - {}) ^ {}", a, b, c),
                            });
                        }
                    }
                }
            }
        }
        
        operations
    }
}