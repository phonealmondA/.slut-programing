use anyhow::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use crate::{MathSolution, VariableAttempt, VariableValue};
use crate::equation_solver::{EquationSolver, Operation};
use rayon::prelude::*;
use evalexpr::*;

pub struct MathEngine {
    solutions: HashMap<String, MathSolution>,
    variable_attempts: HashMap<String, Vec<VariableAttempt>>,
    equation_solver: EquationSolver,
    observation_count: u32,
    function_call_results: HashMap<String, f64>, 
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

        if let Some(cached) = self.solutions.get(&cache_key) {
            if cached.accuracy == 100.0 {
                let cache_time = start_time.elapsed();
                println!("== Using perfect cached solution: {} = {} (100% accuracy)",
                        cached.equation, cached.result);
                println!("   Cache retrieval time: {:?}", cache_time);
                return Ok(cached.clone());
            }
        }

        // Build formula map from previous attempts
        let formula_map = self.build_formula_map(var_name, inputs);
        if !formula_map.is_empty() {
            println!("-- Built formula map with {} entries", formula_map.len());
        }

        let untried_ops = self.get_untried_operations_with_formulas(var_name, inputs, &formula_map);
        println!("-- {} untried operations available for '{}'", untried_ops.len(), var_name);
        
        let solution_start = Instant::now();
        let mut solution = self.find_exact_solution(target, inputs, &untried_ops, var_name)?;
        
        if solution.accuracy < 100.0 {
            println!("!! No exact match found, finding best approximation for target {}", target);
            solution = self.find_best_approximation(target, inputs, &untried_ops, var_name)?;
        }
        
        let solution_time = solution_start.elapsed();
        solution.timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
        
        self.remember_variable_attempt(var_name, &solution);
        
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
        
        if let Ok(value) = expression.parse::<f64>() {
            return Ok(value);
        }
        
        if let Some(var_value) = variables.get(expression) {
            if let VariableValue::Number(n) = var_value {
                println!("-- Resolved variable '{}' = {}", expression, n);
                return Ok(*n);
            }
        }
        
        self.evaluate_arithmetic_expression(expression, variables)
    }
    
    fn parse_calc_parameters(&self, params_str: &str, variables: &HashMap<String, VariableValue>) -> Result<Vec<f64>> {
        let mut params = Vec::new();
        
        for param in params_str.split(',') {
            let param = param.trim();
            
            if let Ok(num) = param.parse::<f64>() {
                params.push(num);
            }
            
            else if let Some(var_value) = variables.get(param) {
                if let VariableValue::Number(n) = var_value {
                    params.push(*n);
                    println!("-- Resolved parameter '{}' = {}", param, n);
                } else {
                    return Err(anyhow::anyhow!("Variable '{}' is not numeric", param));
                }
            }
            
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

        let operations = vec![
            Operation { result: a + b, equation: format!("{} + {}", a, b), formula: format!("{} + {}", a, b) },
            Operation { result: a - b, equation: format!("{} - {}", a, b), formula: format!("{} - {}", a, b) },
            Operation { result: a * b, equation: format!("{} * {}", a, b), formula: format!("{} * {}", a, b) },
            Operation { result: if b != 0.0 { a / b } else { a }, equation: format!("{} / {}", a, b), formula: format!("{} / {}", a, b) },
            Operation { result: a.powf(b), equation: format!("{} ^ {}", a, b), formula: format!("{} ^ {}", a, b) },
        ];

        let chosen = &operations[0];
        println!("   Using operation: {}", chosen.equation);
        chosen.result
    }
    
    fn execute_three_number_calc(&mut self, a: f64, b: f64, c: f64) -> f64 {

        let operations = vec![
            Operation { result: a + b + c, equation: format!("{} + {} + {}", a, b, c), formula: format!("{} + {} + {}", a, b, c) },
            Operation { result: a * b + c, equation: format!("{} * {} + {}", a, b, c), formula: format!("{} * {} + {}", a, b, c) },
            Operation { result: (a + b) * c, equation: format!("({} + {}) * {}", a, b, c), formula: format!("({} + {}) * {}", a, b, c) },
            Operation { result: a + b * c, equation: format!("{} + {} * {}", a, b, c), formula: format!("{} + {} * {}", a, b, c) },
        ];

        let chosen = &operations[0];
        println!("   Using operation: {}", chosen.equation);
        chosen.result
    }
    
    fn evaluate_arithmetic_expression(&self, expression: &str, variables: &HashMap<String, VariableValue>) -> Result<f64> {
        // Create a context with variable values for evalexpr
        let mut context = HashMapContext::new();

        for (var_name, var_value) in variables {
            if let VariableValue::Number(n) = var_value {
                context.set_value(var_name.clone(), Value::from(*n))
                    .map_err(|e| anyhow::anyhow!("Failed to set variable {}: {}", var_name, e))?;
            }
        }

        // Evaluate the expression using evalexpr
        match eval_with_context(expression, &context) {
            Ok(value) => {
                if let Value::Float(f) = value {
                    Ok(f)
                } else if let Value::Int(i) = value {
                    Ok(i as f64)
                } else {
                    Err(anyhow::anyhow!("Expression did not evaluate to a number: {}", expression))
                }
            }
            Err(e) => {
                // Fallback to simple operand resolution if evalexpr fails
                println!("-- evalexpr failed ({}), trying simple resolution", e);
                self.resolve_operand(expression, variables)
            }
        }
    }
    
    fn resolve_operand(&self, operand: &str, variables: &HashMap<String, VariableValue>) -> Result<f64> {
        
        if let Ok(num) = operand.parse::<f64>() {
            return Ok(num);
        }
        
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
    
    /// Build a formula map from inputs and previous attempts
    /// Maps result values to their cumulative formulas
    fn build_formula_map(&self, var_name: &str, inputs: &[f64]) -> HashMap<String, String> {
        let mut formula_map = HashMap::new();

        // Start with inputs as their own formulas (just the numeric value)
        for &input in inputs {
            let key = format!("{:.10}", input);
            formula_map.insert(key, input.to_string());
        }

        // Add formulas from previous attempts
        if let Some(attempts) = self.variable_attempts.get(var_name) {
            for attempt in attempts {
                if let Some(formula) = &attempt.formula {
                    let key = format!("{:.10}", attempt.result);
                    // Only add if we don't already have a formula for this result
                    formula_map.entry(key).or_insert_with(|| formula.clone());
                }
            }
        }

        formula_map
    }

    fn get_untried_operations(&self, var_name: &str, inputs: &[f64]) -> Vec<Operation> {
        let formula_map = HashMap::new();
        self.get_untried_operations_with_formulas(var_name, inputs, &formula_map)
    }

    fn get_untried_operations_with_formulas(&self, var_name: &str, inputs: &[f64], formula_map: &HashMap<String, String>) -> Vec<Operation> {
        let previous_attempts = self.variable_attempts.get(var_name).cloned().unwrap_or_default();
        let attempted_equations: std::collections::HashSet<String> =
            previous_attempts.iter().map(|a| a.equation.clone()).collect();

        println!("-- Variable '{}' has {} previous attempts", var_name, attempted_equations.len());

        let all_operations = self.equation_solver.generate_all_operations_with_formulas(inputs, formula_map);

        all_operations.into_iter()
            .filter(|op| !attempted_equations.contains(&op.equation))
            .collect()
    }
    
    fn find_exact_solution(&self, target: f64, inputs: &[f64], untried_ops: &[Operation], _var_name: &str) -> Result<MathSolution> {
        // Search untried operations in parallel
        if let Some(op) = untried_ops.par_iter().find_any(|op| (op.result - target).abs() < f64::EPSILON) {
            println!("== Exact match found from untried operations: {} = {}", op.equation, target);
            println!("   Formula: {}", op.formula);
            return Ok(MathSolution {
                result: target,
                equation: op.equation.clone(),
                accuracy: 100.0,
                timestamp: 0,
                attempts: 1,
                formula: Some(op.formula.clone()),
            });
        }

        // Search all operations in parallel
        let all_ops = self.equation_solver.generate_all_operations(inputs);
        if let Some(op) = all_ops.par_iter().find_any(|op| (op.result - target).abs() < f64::EPSILON) {
            println!("== Exact match found: {} = {}", op.equation, target);
            println!("   Formula: {}", op.formula);
            return Ok(MathSolution {
                result: target,
                equation: op.equation.clone(),
                accuracy: 100.0,
                timestamp: 0,
                attempts: 1,
                formula: Some(op.formula.clone()),
            });
        }

        Ok(MathSolution {
            result: if !inputs.is_empty() { inputs[0] } else { target },
            equation: if !inputs.is_empty() { inputs[0].to_string() } else { target.to_string() },
            accuracy: 0.0,
            timestamp: 0,
            attempts: 1,
            formula: if !inputs.is_empty() { Some(inputs[0].to_string()) } else { Some(target.to_string()) },
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
                formula: Some(target.to_string()),
            });
        }

        let mut best = MathSolution {
            result: inputs[0],
            equation: inputs[0].to_string(),
            accuracy: self.calculate_accuracy(inputs[0], target),
            timestamp: 0,
            attempts: 1,
            formula: Some(inputs[0].to_string()),
        };

        // Search untried operations in parallel for best match
        if let Some(best_untried) = untried_ops.par_iter()
            .map(|op| {
                let accuracy = self.calculate_accuracy(op.result, target);
                MathSolution {
                    result: op.result,
                    equation: op.equation.clone(),
                    accuracy,
                    timestamp: 0,
                    attempts: 1,
                    formula: Some(op.formula.clone()),
                }
            })
            .max_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap_or(std::cmp::Ordering::Equal))
        {
            if best_untried.accuracy > best.accuracy {
                best = best_untried;
            }
        }

        // Search all operations in parallel for best match
        let all_ops = self.equation_solver.generate_all_operations(inputs);
        if let Some(best_all) = all_ops.par_iter()
            .map(|op| {
                let accuracy = self.calculate_accuracy(op.result, target);
                MathSolution {
                    result: op.result,
                    equation: op.equation.clone(),
                    accuracy,
                    timestamp: 0,
                    attempts: 1,
                    formula: Some(op.formula.clone()),
                }
            })
            .max_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap_or(std::cmp::Ordering::Equal))
        {
            if best_all.accuracy > best.accuracy {
                best = best_all;
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
            formula: solution.formula.clone(),
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