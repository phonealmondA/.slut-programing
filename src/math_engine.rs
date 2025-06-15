// src/math_engine.rs - Enhanced with variable integration and function calls

use anyhow::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use crate::{MathSolution, VariableAttempt, VariableValue};
use crate::equation_solver::{EquationSolver, Operation};

pub struct MathEngine {
    solutions: HashMap<String, MathSolution>,
    variable_attempts: HashMap<String, Vec<VariableAttempt>>,
    equation_solver: EquationSolver,
    observation_count: u32,
    function_call_results: HashMap<String, f64>, // Cache for function call results
}

impl MathEngine {
    pub fn new(
        solutions: HashMap<String, MathSolution>,
        variable_attempts: HashMap<String, Vec<VariableAttempt>>
    ) -> Self {
        Self {
            solutions,
            variable_attempts,
            equation_solver: EquationSolver::new(),
            observation_count: 0,
            function_call_results: HashMap::new(),
        }
    }
    
    pub fn solve_target(&mut self, target: f64, inputs: &[f64], var_name: &str, class_name: &str) -> Result<MathSolution> {
        let start_time = Instant::now();
        self.observation_count += 1;
        
        println!(">> Observation #{} - Target: {} for variable '{}'", 
                self.observation_count, target, var_name);
        
        let cache_key = self.create_cache_key(target, inputs, class_name, var_name);
        
        // Check if we have a perfect cached solution
        if let Some(cached) = self.solutions.get(&cache_key) {
            if cached.accuracy == 100.0 {
                let cache_time = start_time.elapsed();
                println!("== Using perfect cached solution: {} = {} (100% accuracy)", 
                        cached.equation, cached.result);
                println!("   Cache retrieval time: {:?}", cache_time);
                return Ok(cached.clone());
            }
        }
        
        // Get untried operations for this variable
        let untried_ops = self.get_untried_operations(var_name, inputs);
        println!("-- {} untried operations available for '{}'", untried_ops.len(), var_name);
        
        // Try to find exact solution first
        let solution_start = Instant::now();
        let mut solution = self.find_exact_solution(target, inputs, &untried_ops, var_name)?;
        
        // If no exact solution, find best approximation
        if solution.accuracy < 100.0 {
            println!("!! No exact match found, finding best approximation for target {}", target);
            solution = self.find_best_approximation(target, inputs, &untried_ops, var_name)?;
        }
        
        let solution_time = solution_start.elapsed();
        solution.timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
        
        // Remember what this variable tried
        self.remember_variable_attempt(var_name, &solution);
        
        // Cache if it's the best solution so far
        if let Some(cached) = self.solutions.get(&cache_key) {
            if solution.accuracy > cached.accuracy {
                self.solutions.insert(cache_key, solution.clone());
                println!("** New best solution cached: {} = {} (accuracy: {}%)", 
                        solution.equation, solution.result, solution.accuracy);
            }
        } else {
            self.solutions.insert(cache_key, solution.clone());
            println!("** Solution cached: {} = {} (accuracy: {}%)", 
                    solution.equation, solution.result, solution.accuracy);
        }
        
        let total_time = start_time.elapsed();
        println!("   Solution time: {:?}, Total time: {:?}", solution_time, total_time);
        
        Ok(solution)
    }
    
    pub fn solve_expression(&mut self, expression: &str, variables: &HashMap<String, VariableValue>) -> Result<f64> {
        println!(">> Evaluating expression: {}", expression);
        
        // Handle calc() expressions
        if expression.starts_with("calc(") && expression.ends_with(")") {
            let inner = &expression[5..expression.len()-1];
            let params = self.parse_calc_parameters(inner, variables)?;
            
            if params.len() == 2 {
                let result = self.execute_two_number_calc(params[0], params[1]);
                println!("-- calc({}, {}) = {}", params[0], params[1], result);
                return Ok(result);
            } else if params.len() == 3 {
                let result = self.execute_three_number_calc(params[0], params[1], params[2]);
                println!("-- calc({}, {}, {}) = {}", params[0], params[1], params[2], result);
                return Ok(result);
            }
        }
        
        // Handle direct numeric values
        if let Ok(value) = expression.parse::<f64>() {
            return Ok(value);
        }
        
        // Handle variable references
        if let Some(var_value) = variables.get(expression) {
            if let VariableValue::Number(n) = var_value {
                println!("-- Resolved variable '{}' = {}", expression, n);
                return Ok(*n);
            }
        }
        
        // Handle basic arithmetic expressions
        self.evaluate_arithmetic_expression(expression, variables)
    }
    
    fn parse_calc_parameters(&self, params_str: &str, variables: &HashMap<String, VariableValue>) -> Result<Vec<f64>> {
        let mut params = Vec::new();
        
        for param in params_str.split(',') {
            let param = param.trim();
            
            // Try direct number parsing
            if let Ok(num) = param.parse::<f64>() {
                params.push(num);
            }
            // Try variable resolution
            else if let Some(var_value) = variables.get(param) {
                if let VariableValue::Number(n) = var_value {
                    params.push(*n);
                    println!("-- Resolved parameter '{}' = {}", param, n);
                } else {
                    return Err(anyhow::anyhow!("Variable '{}' is not numeric", param));
                }
            }
            // Try function call resolution (future enhancement)
            else if param.contains('(') {
                return Err(anyhow::anyhow!("Function calls in calc() not yet implemented"));
            }
            else {
                return Err(anyhow::anyhow!("Could not resolve parameter: {}", param));
            }
        }
        
        Ok(params)
    }
    
    fn execute_two_number_calc(&mut self, a: f64, b: f64) -> f64 {
        // Use equation solver to find an interesting operation
        let operations = vec![
            Operation { result: a + b, equation: format!("{} + {}", a, b) },
            Operation { result: a - b, equation: format!("{} - {}", a, b) },
            Operation { result: a * b, equation: format!("{} * {}", a, b) },
            Operation { result: if b != 0.0 { a / b } else { a }, equation: format!("{} / {}", a, b) },
            Operation { result: a.powf(b), equation: format!("{} ^ {}", a, b) },
        ];
        
        // For now, use addition, but could be made more intelligent
        let chosen = &operations[0]; // Addition
        println!("   Using operation: {}", chosen.equation);
        chosen.result
    }
    
    fn execute_three_number_calc(&mut self, a: f64, b: f64, c: f64) -> f64 {
        // More complex three-number operations
        let operations = vec![
            Operation { result: a + b + c, equation: format!("{} + {} + {}", a, b, c) },
            Operation { result: a * b + c, equation: format!("{} * {} + {}", a, b, c) },
            Operation { result: (a + b) * c, equation: format!("({} + {}) * {}", a, b, c) },
            Operation { result: a + b * c, equation: format!("{} + {} * {}", a, b, c) },
        ];
        
        // Use the first operation for now
        let chosen = &operations[0];
        println!("   Using operation: {}", chosen.equation);
        chosen.result
    }
    
    fn evaluate_arithmetic_expression(&self, expression: &str, variables: &HashMap<String, VariableValue>) -> Result<f64> {
        // Simple arithmetic parser - this could be much more sophisticated
        if expression.contains('+') {
            let parts: Vec<&str> = expression.split('+').collect();
            if parts.len() == 2 {
                let left = self.resolve_operand(parts[0].trim(), variables)?;
                let right = self.resolve_operand(parts[1].trim(), variables)?;
                return Ok(left + right);
            }
        }
        
        if expression.contains('-') {
            let parts: Vec<&str> = expression.split('-').collect();
            if parts.len() == 2 {
                let left = self.resolve_operand(parts[0].trim(), variables)?;
                let right = self.resolve_operand(parts[1].trim(), variables)?;
                return Ok(left - right);
            }
        }
        
        if expression.contains('*') {
            let parts: Vec<&str> = expression.split('*').collect();
            if parts.len() == 2 {
                let left = self.resolve_operand(parts[0].trim(), variables)?;
                let right = self.resolve_operand(parts[1].trim(), variables)?;
                return Ok(left * right);
            }
        }
        
        if expression.contains('/') {
            let parts: Vec<&str> = expression.split('/').collect();
            if parts.len() == 2 {
                let left = self.resolve_operand(parts[0].trim(), variables)?;
                let right = self.resolve_operand(parts[1].trim(), variables)?;
                if right != 0.0 {
                    return Ok(left / right);
                }
            }
        }
        
        // Fallback: try to resolve as single operand
        self.resolve_operand(expression, variables)
    }
    
    fn resolve_operand(&self, operand: &str, variables: &HashMap<String, VariableValue>) -> Result<f64> {
        // Try number parsing
        if let Ok(num) = operand.parse::<f64>() {
            return Ok(num);
        }
        
        // Try variable resolution
        if let Some(var_value) = variables.get(operand) {
            if let VariableValue::Number(n) = var_value {
                return Ok(*n);
            }
        }
        
        Err(anyhow::anyhow!("Could not resolve operand: {}", operand))
    }
    
    fn create_cache_key(&self, target: f64, inputs: &[f64], class_name: &str, var_name: &str) -> String {
        let inputs_str = inputs.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        format!("{}-{}-{}-{}", class_name, var_name, target, inputs_str)
    }
    
    fn get_untried_operations(&self, var_name: &str, inputs: &[f64]) -> Vec<Operation> {
        let previous_attempts = self.variable_attempts.get(var_name).cloned().unwrap_or_default();
        let attempted_equations: std::collections::HashSet<String> = 
            previous_attempts.iter().map(|a| a.equation.clone()).collect();
        
        println!("-- Variable '{}' has {} previous attempts", var_name, attempted_equations.len());
        
        // Generate all possible operations
        let all_operations = self.equation_solver.generate_all_operations(inputs);
        
        // Filter out already tried equations
        all_operations.into_iter()
            .filter(|op| !attempted_equations.contains(&op.equation))
            .collect()
    }
    
    fn find_exact_solution(&self, target: f64, inputs: &[f64], untried_ops: &[Operation], _var_name: &str) -> Result<MathSolution> {
        // Try untried operations first
        for op in untried_ops {
            if (op.result - target).abs() < f64::EPSILON {
                println!("== Exact match found from untried operations: {} = {}", op.equation, target);
                return Ok(MathSolution {
                    result: target,
                    equation: op.equation.clone(),
                    accuracy: 100.0,
                    timestamp: 0, // Will be set by caller
                    attempts: 1,
                });
            }
        }
        
        // Try all possible operations if not found in untried
        let all_ops = self.equation_solver.generate_all_operations(inputs);
        for op in &all_ops {
            if (op.result - target).abs() < f64::EPSILON {
                println!("== Exact match found: {} = {}", op.equation, target);
                return Ok(MathSolution {
                    result: target,
                    equation: op.equation.clone(),
                    accuracy: 100.0,
                    timestamp: 0,
                    attempts: 1,
                });
            }
        }
        
        // No exact solution found
        Ok(MathSolution {
            result: if !inputs.is_empty() { inputs[0] } else { target },
            equation: if !inputs.is_empty() { inputs[0].to_string() } else { target.to_string() },
            accuracy: 0.0,
            timestamp: 0,
            attempts: 1,
        })
    }
    
    fn find_best_approximation(&self, target: f64, inputs: &[f64], untried_ops: &[Operation], _var_name: &str) -> Result<MathSolution> {
        if inputs.is_empty() {
            return Ok(MathSolution {
                result: target,
                equation: target.to_string(),
                accuracy: 100.0,
                timestamp: 0,
                attempts: 1,
            });
        }
        
        let mut best = MathSolution {
            result: inputs[0],
            equation: inputs[0].to_string(),
            accuracy: self.calculate_accuracy(inputs[0], target),
            timestamp: 0,
            attempts: 1,
        };
        
        // Check untried operations first
        for op in untried_ops {
            let accuracy = self.calculate_accuracy(op.result, target);
            if accuracy > best.accuracy {
                best = MathSolution {
                    result: op.result,
                    equation: op.equation.clone(),
                    accuracy,
                    timestamp: 0,
                    attempts: 1,
                };
            }
        }
        
        // Try all other combinations if needed
        let all_ops = self.equation_solver.generate_all_operations(inputs);
        for op in &all_ops {
            let accuracy = self.calculate_accuracy(op.result, target);
            if accuracy > best.accuracy {
                best = MathSolution {
                    result: op.result,
                    equation: op.equation.clone(),
                    accuracy,
                    timestamp: 0,
                    attempts: 1,
                };
            }
        }
        
        println!("== Best approximation: {} = {} (accuracy: {}%)", 
                best.equation, best.result, best.accuracy);
        
        Ok(best)
    }
    
    fn calculate_accuracy(&self, actual: f64, target: f64) -> f64 {
        if (actual - target).abs() < f64::EPSILON {
            return 100.0;
        }
        if !actual.is_finite() || actual.is_nan() {
            return 0.0;
        }
        
        let diff = (actual - target).abs();
        let reference = target.abs().max(100.0);
        let accuracy = (1.0 - diff / reference) * 100.0;
        accuracy.max(0.0).min(100.0)
    }
    
    fn remember_variable_attempt(&mut self, var_name: &str, solution: &MathSolution) {
        let attempt = VariableAttempt {
            equation: solution.equation.clone(),
            result: solution.result,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap_or_default().as_millis() as u64,
            accuracy: solution.accuracy,
        };
        
        self.variable_attempts
            .entry(var_name.to_string())
            .or_insert_with(Vec::new)
            .push(attempt);
    }
    
    pub fn get_solutions(&self) -> HashMap<String, MathSolution> {
        self.solutions.clone()
    }
    
    pub fn get_variable_attempts(&self) -> HashMap<String, Vec<VariableAttempt>> {
        self.variable_attempts.clone()
    }
    
    pub fn store_function_result(&mut self, function_name: &str, result: f64) {
        self.function_call_results.insert(function_name.to_string(), result);
        println!("++ Cached function result: {}() = {}", function_name, result);
    }
    
    pub fn get_function_result(&self, function_name: &str) -> Option<f64> {
        self.function_call_results.get(function_name).copied()
    }
}