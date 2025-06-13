use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapsedState {
    pub result: f64,
    pub equation: String,
    pub accuracy: f64,
    pub timestamp: u64,
    pub calculation_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableAttempt {
    pub equation: String,
    pub result: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct TargetSolution {
    pub result: f64,
    pub equation: String,
    pub accuracy: f64,
    pub calculation_time: f64,
}

#[derive(Debug, Clone)]
struct MathOperation {
    result: f64,
    equation: String,
}

pub struct QuantumTargetSeeker {
    collapsed_states: HashMap<String, CollapsedState>,
    variable_attempts: HashMap<String, Vec<VariableAttempt>>,
    observation_count: u32,
}

impl QuantumTargetSeeker {
    pub fn new(
        collapsed_states: HashMap<String, CollapsedState>,
        variable_attempts: HashMap<String, Vec<VariableAttempt>>,
    ) -> Self {
        Self {
            collapsed_states,
            variable_attempts,
            observation_count: 0,
        }
    }
    
    pub fn find_target_solution(
        &mut self,
        target: f64,
        inputs: &[f64],
        class_name: &str,
        var_name: &str,
    ) -> Result<TargetSolution> {
        let start_time = Instant::now();
        self.observation_count += 1;
        
        println!(">> Observation #{} - Target: {} for variable '{}'", 
                self.observation_count, target, var_name);
        
        // Check for cached exact solution
        let cache_key = format!("{}-{}-{}", class_name, target, 
                               inputs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        
        if let Some(cached) = self.collapsed_states.get(&cache_key) {
            if cached.accuracy == 100.0 {
                println!("== Using perfect cached state: {} = {} (100% accuracy)", 
                        cached.equation, cached.result);
                return Ok(TargetSolution {
                    result: cached.result,
                    equation: cached.equation.clone(),
                    accuracy: cached.accuracy,
                    calculation_time: start_time.elapsed().as_millis() as f64,
                });
            }
        }
        
        // Get untried operations for this variable
        let untried_ops = self.get_untried_operations(var_name, inputs);
        println!("-- {} untried operations available for '{}'", untried_ops.len(), var_name);
        
        // Try to find exact solution
        if let Some(exact_solution) = self.find_exact_solution(target, &untried_ops) {
            println!("== Exact match found: {} = {}", exact_solution.equation, exact_solution.result);
            self.remember_variable_attempt(var_name, &exact_solution.equation, exact_solution.result);
            return Ok(TargetSolution {
                result: exact_solution.result,
                equation: exact_solution.equation,
                accuracy: 100.0,
                calculation_time: start_time.elapsed().as_millis() as f64,
            });
        }
        
        // Find best approximation
        let best_approximation = self.find_best_approximation(target, inputs, &untried_ops);
        println!("== Best approximation: {} = {} (accuracy: {}%)", 
                best_approximation.equation, best_approximation.result, best_approximation.accuracy);
        
        self.remember_variable_attempt(var_name, &best_approximation.equation, best_approximation.result);
        
        Ok(TargetSolution {
            result: best_approximation.result,
            equation: best_approximation.equation,
            accuracy: best_approximation.accuracy,
            calculation_time: start_time.elapsed().as_millis() as f64,
        })
    }
    
    fn get_untried_operations(&self, var_name: &str, inputs: &[f64]) -> Vec<MathOperation> {
        let key = format!("var_{}", var_name);
        let previous_attempts = self.variable_attempts.get(&key).cloned().unwrap_or_default();
        let attempted_equations: std::collections::HashSet<String> = 
            previous_attempts.iter().map(|a| a.equation.clone()).collect();
        
        let all_operations = self.generate_all_operations(inputs);
        all_operations.into_iter()
            .filter(|op| !attempted_equations.contains(&op.equation))
            .collect()
    }
    
    fn generate_all_operations(&self, inputs: &[f64]) -> Vec<MathOperation> {
        let mut operations = Vec::new();
        
        // Single number operations
        for &num in inputs {
            operations.push(MathOperation {
                result: num,
                equation: num.to_string(),
            });
        }
        
        // Two-number operations
        for i in 0..inputs.len() {
            for j in i + 1..inputs.len() {
                let a = inputs[i];
                let b = inputs[j];
                
                operations.extend(vec![
                    MathOperation { result: a + b, equation: format!("{} + {}", a, b) },
                    MathOperation { result: a - b, equation: format!("{} - {}", a, b) },
                    MathOperation { result: b - a, equation: format!("{} - {}", b, a) },
                    MathOperation { result: a * b, equation: format!("{} * {}", a, b) },
                ]);
                
                if b != 0.0 {
                    operations.push(MathOperation { result: a / b, equation: format!("{} / {}", a, b) });
                }
                if a != 0.0 {
                    operations.push(MathOperation { result: b / a, equation: format!("{} / {}", b, a) });
                }
                
                operations.extend(vec![
                    MathOperation { result: a.powf(b), equation: format!("{} ^ {}", a, b) },
                    MathOperation { result: b.powf(a), equation: format!("{} ^ {}", b, a) },
                    MathOperation { result: a.max(b), equation: format!("max({}, {})", a, b) },
                    MathOperation { result: a.min(b), equation: format!("min({}, {})", a, b) },
                ]);
            }
        }
        
        // Three-number operations
        if inputs.len() >= 3 {
            for i in 0..inputs.len() - 2 {
                for j in i + 1..inputs.len() - 1 {
                    for k in j + 1..inputs.len() {
                        let a = inputs[i];
                        let b = inputs[j];
                        let c = inputs[k];
                        
                        operations.extend(vec![
                            MathOperation { result: a + b + c, equation: format!("{} + {} + {}", a, b, c) },
                            MathOperation { result: a + b - c, equation: format!("{} + {} - {}", a, b, c) },
                            MathOperation { result: a - b + c, equation: format!("{} - {} + {}", a, b, c) },
                            MathOperation { result: a - b - c, equation: format!("{} - {} - {}", a, b, c) },
                            MathOperation { result: a * b + c, equation: format!("{} * {} + {}", a, b, c) },
                            MathOperation { result: a * b - c, equation: format!("{} * {} - {}", a, b, c) },
                            MathOperation { result: a + b * c, equation: format!("{} + {} * {}", a, b, c) },
                            MathOperation { result: a - b * c, equation: format!("{} - {} * {}", a, b, c) },
                            MathOperation { result: (a + b) * c, equation: format!("({} + {}) * {}", a, b, c) },
                            MathOperation { result: (a - b) * c, equation: format!("({} - {}) * {}", a, b, c) },
                            MathOperation { result: a * (b + c), equation: format!("{} * ({} + {})", a, b, c) },
                            MathOperation { result: a * (b - c), equation: format!("{} * ({} - {})", a, b, c) },
                            MathOperation { result: a * b * c, equation: format!("{} * {} * {}", a, b, c) },
                        ]);
                        
                        if c != 0.0 {
                            operations.extend(vec![
                                MathOperation { result: (a + b) / c, equation: format!("({} + {}) / {}", a, b, c) },
                                MathOperation { result: (a - b) / c, equation: format!("({} - {}) / {}", a, b, c) },
                            ]);
                        }
                        
                        if b != 0.0 {
                            operations.extend(vec![
                                MathOperation { result: a / b + c, equation: format!("{} / {} + {}", a, b, c) },
                                MathOperation { result: a / b - c, equation: format!("{} / {} - {}", a, b, c) },
                            ]);
                        }
                        
                        operations.extend(vec![
                            MathOperation { result: a.powf(b) + c, equation: format!("{} ^ {} + {}", a, b, c) },
                            MathOperation { result: a.powf(b) - c, equation: format!("{} ^ {} - {}", a, b, c) },
                            MathOperation { result: (a + b).powf(c), equation: format!("({} + {}) ^ {}", a, b, c) },
                            MathOperation { result: (a - b).powf(c), equation: format!("({} - {}) ^ {}", a, b, c) },
                        ]);
                    }
                }
            }
        }
        
        // Filter out invalid results
        operations.into_iter()
            .filter(|op| op.result.is_finite() && !op.result.is_nan())
            .collect()
    }
    
    fn find_exact_solution(&self, target: f64, operations: &[MathOperation]) -> Option<MathOperation> {
        operations.iter()
            .find(|op| (op.result - target).abs() < f64::EPSILON)
            .cloned()
    }
    
    fn find_best_approximation(&self, target: f64, inputs: &[f64], untried_ops: &[MathOperation]) -> TargetSolution {
        let mut best_op = MathOperation {
            result: inputs[0],
            equation: inputs[0].to_string(),
        };
        let mut best_accuracy = self.calculate_accuracy(inputs[0], target);
        
        // Check untried operations first
        for op in untried_ops {
            let accuracy = self.calculate_accuracy(op.result, target);
            if accuracy > best_accuracy {
                best_op = op.clone();
                best_accuracy = accuracy;
            }
        }
        
        // If untried operations didn't give us something good, try all operations
        if best_accuracy < 90.0 {
            let all_ops = self.generate_all_operations(inputs);
            for op in all_ops {
                let accuracy = self.calculate_accuracy(op.result, target);
                if accuracy > best_accuracy {
                    best_op = op;
                    best_accuracy = accuracy;
                }
            }
        }
        
        TargetSolution {
            result: best_op.result,
            equation: best_op.equation,
            accuracy: best_accuracy,
            calculation_time: 0.0, // Will be set by caller
        }
    }
    
    fn calculate_accuracy(&self, actual: f64, target: f64) -> f64 {
        if (actual - target).abs() < f64::EPSILON {
            return 100.0;
        }
        if !actual.is_finite() || actual.is_nan() {
            return 0.0;
        }
        let diff = (actual - target).abs();
        let max_val = target.abs().max(100.0);
        ((1.0 - diff / max_val) * 100.0).max(0.0)
    }
    
    fn remember_variable_attempt(&mut self, var_name: &str, equation: &str, result: f64) {
        let key = format!("var_{}", var_name);
        let attempt = VariableAttempt {
            equation: equation.to_string(),
            result,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        
        self.variable_attempts.entry(key).or_insert_with(Vec::new).push(attempt);
    }

    
    pub fn get_collapsed_states(&self) -> HashMap<String, CollapsedState> {
        self.collapsed_states.clone()
    }
    
    pub fn get_variable_attempts(&self) -> HashMap<String, Vec<VariableAttempt>> {
        self.variable_attempts.clone()
    }

}