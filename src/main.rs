// src/main.rs - Enhanced with string interpolation, variable storage, and function hierarchy

use anyhow::Result;
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

mod function_builder;
mod function_executor;
mod math_engine;
mod equation_solver;
mod variable_manager;

use function_builder::FunctionBuilder;
use function_executor::FunctionExecutor;
use math_engine::MathEngine;
use variable_manager::VariableManager;

#[derive(Parser)]
#[command(name = "quantum")]
#[command(about = "Quantum Consciousness Programming Language Transpiler")]
struct Args {
    /// Input .slut file
    file: PathBuf,
    
    /// Number of observations (1-10)
    #[arg(short, long, default_value = "1")]
    observations: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuantumCache {
    templates: HashMap<String, CachedTemplate>,
    variables: HashMap<String, StoredVariable>,
    quantum_states: HashMap<String, CollapsedState>,
    variable_attempts: HashMap<String, Vec<VariableAttempt>>,
    built_functions: HashMap<String, BuiltFunction>,
    math_solutions: HashMap<String, MathSolution>,
    function_results: HashMap<String, FunctionResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredVariable {
    pub name: String,
    pub value: VariableValue,
    pub timestamp: u64,
    pub source_equation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VariableValue {
    Number(f64),
    String(String),
    Boolean(bool),
    FunctionResult(String), // Store function name that generated this
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionResult {
    pub function_name: String,
    pub result: VariableValue,
    pub execution_time: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedTemplate {
    name: String,
    func_type: String,
    parameter_count: usize,
    timestamp: u64,
    is_built: bool,
    file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltFunction {
    pub name: String,
    pub variants: Vec<FunctionVariant>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionVariant {
    pub parameter_count: usize,
    pub parameter_pattern: String,
    pub rust_function_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CollapsedState {
    result: f64,
    equation: String,
    accuracy: f64,
    timestamp: u64,
    calculation_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariableAttempt {
    equation: String,
    result: f64,
    timestamp: u64,
    accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathSolution {
    pub result: f64,
    pub equation: String,
    pub accuracy: f64,
    pub timestamp: u64,
    pub attempts: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("** Quantum Consciousness Observer (Rust Edition - Enhanced)");
    println!(">> Building programs with variable storage, string interpolation, and function hierarchy");
    println!(">> Executing: {:?}", args.file);
    
    let mut transpiler = QuantumTranspiler::new()?;
    
    for i in 1..=args.observations {
        if args.observations > 1 {
            println!("== OBSERVATION {} ==", i);
        }
        
        transpiler.execute_file(&args.file)?;
        
        if i < args.observations {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
    
    println!("** Complete!");
    Ok(())
}

struct QuantumTranspiler {
    cache: QuantumCache,
    execution_count: u32,
    function_builder: FunctionBuilder,
    function_executor: FunctionExecutor,
    math_engine: MathEngine,
    variable_manager: VariableManager,
    current_class_name: String,
}

impl QuantumTranspiler {
    fn new() -> Result<Self> {
        let cache = Self::load_cache().unwrap_or_else(|_| {
            println!("** Starting with fresh quantum consciousness cache");
            QuantumCache {
                templates: HashMap::new(),
                variables: HashMap::new(),
                quantum_states: HashMap::new(),
                variable_attempts: HashMap::new(),
                built_functions: HashMap::new(),
                math_solutions: HashMap::new(),
                function_results: HashMap::new(),
            }
        });
        
        if !cache.templates.is_empty() || !cache.built_functions.is_empty() || !cache.math_solutions.is_empty() {
            println!("** Loaded previous quantum states, built functions, variables, and math solutions from cache");
        }
        
        let function_builder = FunctionBuilder::new()?;
        let function_executor = FunctionExecutor::new()?;
        let math_engine = MathEngine::new(cache.math_solutions.clone(), cache.variable_attempts.clone());
        let variable_manager = VariableManager::new(cache.variables.clone());
        
        Ok(Self {
            cache,
            execution_count: 0,
            function_builder,
            function_executor,
            math_engine,
            variable_manager,
            current_class_name: String::new(),
        })
    }
    
    fn load_cache() -> Result<QuantumCache> {
        let content = fs::read_to_string("quantum_consciousness_cache.json")?;
        let cache: QuantumCache = serde_json::from_str(&content)?;
        Ok(cache)
    }
    
    fn save_cache(&mut self) -> Result<()> {
        // Update cache with latest data
        self.cache.math_solutions = self.math_engine.get_solutions();
        self.cache.variable_attempts = self.math_engine.get_variable_attempts();
        self.cache.variables = self.variable_manager.get_all_variables();
        
        let content = serde_json::to_string_pretty(&self.cache)?;
        fs::write("quantum_consciousness_cache.json", content)?;
        Ok(())
    }
    
    fn execute_file(&mut self, file_path: &PathBuf) -> Result<()> {
        let source = fs::read_to_string(file_path)?;
        self.parse_and_execute(&source)?;
        self.save_cache()?;
        Ok(())
    }
    
    fn parse_and_execute(&mut self, source: &str) -> Result<()> {
        println!(">> Building program from your intentions...");
        
        // Extract all classes first for function hierarchy
        self.extract_all_classes(source)?;
        
        let main_regex = Regex::new(r"\*\s*<main>\s*(\w+)\s*\{[^}]*\^\s*observe_execution\s*\{([\s\S]*?)\}\s*\}")?;
        
        if let Some(captures) = main_regex.captures(source) {
            let class_name = &captures[1];
            let body = &captures[2];
            
            println!(">> Quantum consciousness activated for: {}", class_name);
            self.current_class_name = class_name.to_string();
            self.execute_main_body(body, class_name)?;
            println!("** Program built and executed successfully!");
        } else {
            println!("!! No main class found in source");
        }
        
        Ok(())
    }
    
    fn extract_all_classes(&mut self, source: &str) -> Result<()> {
        let class_regex = Regex::new(r"\*\s*(?:<main>\s*)?(\w+)\s*(?:\(\[([^\]]*)\]\))?\s*\{[\s\S]*?\^\s*observe_execution\s*\{([\s\S]*?)\}\s*\}")?;
        
        for captures in class_regex.captures_iter(source) {
            let class_name = &captures[1];
            let _parameters = captures.get(2).map(|m| m.as_str()).unwrap_or("");
            let body = &captures[3];
            
            if !captures[0].contains("<main>") {
                // Store non-main classes as callable functions
                println!(">> Discovered function class: {}", class_name);
                self.cache.function_results.insert(
                    class_name.to_string(),
                    FunctionResult {
                        function_name: class_name.to_string(),
                        result: VariableValue::String(body.to_string()),
                        execution_time: 0.0,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
                    }
                );
            }
        }
        
        Ok(())
    }
    
    fn execute_main_body(&mut self, body: &str, class_name: &str) -> Result<()> {
        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            
            self.execute_statement(line, class_name)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, statement: &str, class_name: &str) -> Result<()> {
        // String interpolation in speak statements: speak("Result: ~variable~")
        let speak_interpolation_regex = Regex::new(r#"speak\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = speak_interpolation_regex.captures(statement) {
            let message = &captures[1];
            let interpolated = self.interpolate_string(message)?;
            println!("{}", interpolated);
            return Ok(());
        }
        
        // Variable assignment with function calls: result <> someFunction()
        let var_function_regex = Regex::new(r"(\w+)\s*<>\s*(\w+)\s*\(\s*\)")?;
        if let Some(captures) = var_function_regex.captures(statement) {
            let var_name = &captures[1];
            let function_name = &captures[2];
            return self.execute_function_call_assignment(var_name, function_name, class_name);
        }
        
        // Variable assignment with expressions: result <> calc(1, 2)
        let var_expression_regex = Regex::new(r"(\w+)\s*<>\s*(.+)")?;
        if let Some(captures) = var_expression_regex.captures(statement) {
            let var_name = &captures[1];
            let expression = &captures[2];
            return self.execute_variable_assignment(var_name, expression, class_name);
        }
        
        // Target-seeking mathematics: result([56]) <> randomChoice([1, 2, 3, 55])
        let target_math_regex = Regex::new(r"(\w+)\s*\(\s*\[\s*([^\]]+)\s*\]\s*\)\s*<>\s*randomChoice\s*\(\s*\[\s*([^\]]*)\s*\]\s*\)")?;
        if let Some(captures) = target_math_regex.captures(statement) {
            let var_name = &captures[1];
            let target_str = &captures[2];
            let inputs_str = &captures[3];
            return self.solve_target_math(var_name, target_str, inputs_str, class_name);
        }
        
        // Function synthesis: smartLoop(params) <> function(loop)
        let poly_synthesis_regex = Regex::new(r"(\w+)\s*\(\s*([^)]*)\s*\)\s*<>\s*function\s*\(\s*(\w+)\s*\)")?;
        if let Some(captures) = poly_synthesis_regex.captures(statement) {
            let func_name = &captures[1];
            let params = &captures[2];
            let func_type = &captures[3];
            return self.synthesize_polymorphic_function(func_name, params, func_type);
        }
        
        // Function execution: smartLoop(3)("console.log('hello')")
        let poly_exec_regex = Regex::new(r#"(\w+)\s*\(\s*([^)]+)\s*\)\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = poly_exec_regex.captures(statement) {
            let func_name = &captures[1];
            let params = &captures[2];
            let body = &captures[3];
            return self.execute_polymorphic_function(func_name, params, body);
        }
        
        // Variable output: woof variable_name
        let woof_regex = Regex::new(r"woof\s+(\w+)")?;
        if let Some(captures) = woof_regex.captures(statement) {
            let var_name = &captures[1];
            return self.output_variable(var_name);
        }
        
        Ok(())
    }
    
    fn interpolate_string(&self, message: &str) -> Result<String> {
        let var_regex = Regex::new(r"~(\w+)~")?;
        let mut result = message.to_string();
        
        for captures in var_regex.captures_iter(message) {
            let var_name = &captures[1];
            let placeholder = &captures[0];
            
            if let Some(variable) = self.variable_manager.get_variable(var_name) {
                let value_str = match &variable.value {
                    VariableValue::Number(n) => n.to_string(),
                    VariableValue::String(s) => s.clone(),
                    VariableValue::Boolean(b) => b.to_string(),
                    VariableValue::FunctionResult(f) => format!("[Function: {}]", f),
                };
                result = result.replace(placeholder, &value_str);
            } else {
                result = result.replace(placeholder, &format!("[undefined: {}]", var_name));
            }
        }
        
        Ok(result)
    }
    
    fn execute_function_call_assignment(&mut self, var_name: &str, function_name: &str, _class_name: &str) -> Result<()> {
        println!(">> Executing function call: {} = {}()", var_name, function_name);
        
        // Check if it's a built-in function class
        if let Some(_function_result) = self.cache.function_results.get(function_name) {
            let start_time = std::time::Instant::now();
            
            // Actually execute the function body and get the result
            let function_result = self.execute_function_body(function_name)?;
            let execution_time = start_time.elapsed().as_secs_f64() * 1000.0;
            
            self.variable_manager.store_variable(
                var_name,
                function_result,
                Some(format!("{}()", function_name)),
            )?;
            
            println!("-- Function {} executed and stored in variable '{}'", function_name, var_name);
            println!("   Execution time: {:.3}ms", execution_time);
        } else {
            println!("!! Function {} not found", function_name);
        }
        
        Ok(())
    }
    
    fn execute_function_body(&mut self, function_name: &str) -> Result<VariableValue> {
        println!(">> Executing function body: {}", function_name);
        
        // Find the function body in the cache
        if let Some(function_result) = self.cache.function_results.get(function_name) {
            let body = match &function_result.result {
                VariableValue::String(body_str) => body_str.clone(),
                _ => return Ok(VariableValue::String(format!("Invalid function: {}", function_name))),
            };
            
            // Create a temporary variable scope for this function
            let mut function_return_value = VariableValue::Number(0.0);
            
            // Execute each statement in the function body
            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() { continue; }
                
                // Handle woof statements (function returns)
                let woof_regex = Regex::new(r"woof\s+(\w+)")?;
                if let Some(captures) = woof_regex.captures(line) {
                    let return_var = &captures[1];
                    
                    // Get the value of the variable to return
                    if let Some(variable) = self.variable_manager.get_variable(return_var) {
                        function_return_value = variable.value.clone();
                        println!("-- Function {} returning: {:?}", function_name, function_return_value);
                        break;
                    } else {
                        println!("!! Return variable '{}' not found in function {}", return_var, function_name);
                    }
                } else {
                    // Execute other statements within the function context
                    self.execute_statement(line, function_name)?;
                }
            }
            
            Ok(function_return_value)
        } else {
            Err(anyhow::anyhow!("Function {} not found", function_name))
        }
    }
    
    fn execute_variable_assignment(&mut self, var_name: &str, expression: &str, _class_name: &str) -> Result<()> {
        println!(">> Variable assignment: {} <> {}", var_name, expression);
        
        // Handle different types of expressions
        if expression.starts_with("calc(") && expression.ends_with(")") {
            // Mathematical calculation with variable support
            let inner = &expression[5..expression.len()-1]; // Remove "calc(" and ")"
            let params: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            
            if params.len() >= 2 {
                let mut resolved_params = Vec::new();
                
                // Resolve each parameter (could be number or variable)
                for param in &params {
                    if let Ok(num) = param.parse::<f64>() {
                        // Direct number
                        resolved_params.push(num);
                    } else if let Some(variable) = self.variable_manager.get_variable(param) {
                        // Variable reference
                        if let VariableValue::Number(n) = variable.value {
                            resolved_params.push(n);
                            println!("-- Resolved variable '{}' = {}", param, n);
                        } else {
                            println!("!! Variable '{}' is not numeric", param);
                            return Ok(());
                        }
                    } else {
                        println!("!! Could not resolve parameter: {}", param);
                        return Ok(());
                    }
                }
                
                // Perform calculation
                let result = if resolved_params.len() == 2 {
                    resolved_params[0] + resolved_params[1] // Simple addition for now
                } else if resolved_params.len() == 3 {
                    resolved_params[0] + resolved_params[1] + resolved_params[2] // Three-number addition
                } else {
                    resolved_params.iter().sum() // Sum all parameters
                };
                
                self.variable_manager.store_variable(
                    var_name,
                    VariableValue::Number(result),
                    Some(format!("calc({})", inner)),
                )?;
                
                println!("-- Calculated and stored: {} = {} (from calc({}))", var_name, result, inner);
            } else {
                println!("!! calc() requires at least 2 parameters");
            }
        } else if expression.starts_with("randomChoice(") {
            // Random choice from array
            let choice_regex = Regex::new(r"randomChoice\s*\(\s*\[\s*([^\]]*)\s*\]\s*\)")?;
            if let Some(captures) = choice_regex.captures(expression) {
                let choices_str = &captures[1];
                let choice_parts: Vec<&str> = choices_str.split(',').map(|s| s.trim()).collect();
                
                let mut resolved_choices = Vec::new();
                
                // Resolve each choice (could be number, string, or variable)
                for choice in &choice_parts {
                    if let Ok(num) = choice.parse::<f64>() {
                        // Direct number
                        resolved_choices.push(VariableValue::Number(num));
                    } else if let Some(variable) = self.variable_manager.get_variable(choice) {
                        // Variable reference
                        resolved_choices.push(variable.value.clone());
                        println!("-- Resolved choice variable '{}' = {:?}", choice, variable.value);
                    } else {
                        // String literal
                        resolved_choices.push(VariableValue::String(choice.trim_matches('"').to_string()));
                    }
                }
                
                if !resolved_choices.is_empty() {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let chosen = &resolved_choices[rng.gen_range(0..resolved_choices.len())];
                    
                    self.variable_manager.store_variable(
                        var_name,
                        chosen.clone(),
                        Some(format!("randomChoice({})", choices_str)),
                    )?;
                    
                    println!("-- Random choice stored: {} = {:?}", var_name, chosen);
                }
            }
        } else {
            // Direct value assignment
            let value = if let Ok(num) = expression.parse::<f64>() {
                VariableValue::Number(num)
            } else if expression == "true" || expression == "false" {
                VariableValue::Boolean(expression == "true")
            } else {
                VariableValue::String(expression.trim_matches('"').to_string())
            };
            
            self.variable_manager.store_variable(var_name, value, None)?;
            println!("-- Direct assignment: {} = {}", var_name, expression);
        }
        
        Ok(())
    }
    
    fn output_variable(&self, var_name: &str) -> Result<()> {
        if let Some(variable) = self.variable_manager.get_variable(var_name) {
            match &variable.value {
                VariableValue::Number(n) => println!("Final result: {}", n),
                VariableValue::String(s) => println!("Final result: {}", s),
                VariableValue::Boolean(b) => println!("Final result: {}", b),
                VariableValue::FunctionResult(f) => println!("Final result: [Function: {}]", f),
            }
            
            if let Some(eq) = &variable.source_equation {
                println!("   Source: {}", eq);
            }
        } else {
            println!("!! Variable '{}' not found", var_name);
        }
        Ok(())
    }
    
    fn solve_target_math(&mut self, var_name: &str, target_str: &str, inputs_str: &str, class_name: &str) -> Result<()> {
        let target: f64 = target_str.parse()?;
        
        // Use variable manager to resolve inputs (supports both numbers and variables)
        let inputs = self.variable_manager.resolve_expression_inputs(inputs_str);
        
        println!(">> Target-seeking quantum mathematics for variable '{}': target={}, inputs={:?}", 
                var_name, target, inputs);
        
        let solution = self.math_engine.solve_target(target, &inputs, var_name, class_name)?;
        
        // Store the result as a variable
        self.variable_manager.store_variable(
            var_name,
            VariableValue::Number(solution.result),
            Some(solution.equation.clone()),
        )?;
        
        println!("== Solution found: {} = {} (accuracy: {}%)", 
                solution.equation, solution.result, solution.accuracy);
        println!("-- Variable '{}' stored with value: {}", var_name, solution.result);
        
        Ok(())
    }
    
    fn synthesize_polymorphic_function(&mut self, name: &str, params: &str, func_type: &str) -> Result<()> {
        let param_count = if params.trim().is_empty() { 0 } else { params.split(',').count() };
        let cache_key = format!("{}_{}_{}", name, func_type, param_count);
        
        println!(">> Synthesizing {} function: {} (supports {} parameters)", func_type, name, param_count);
        
        if let Some(template) = self.cache.templates.get(&cache_key) {
            if template.is_built {
                println!("== Using previously built function: {}", name);
                return Ok(());
            }
        }
        
        println!(">> Generating Rust code for function: {}", name);
        let built_function = self.function_builder.build_function(name, func_type, param_count)?;
        
        let template = CachedTemplate {
            name: name.to_string(),
            func_type: func_type.to_string(),
            parameter_count: param_count,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
            is_built: true,
            file_path: Some(format!("functions/src/{}.rs", name.to_lowercase())),
        };
        
        self.cache.templates.insert(cache_key, template);
        self.cache.built_functions.insert(name.to_string(), built_function);
        
        println!("** Function successfully built and cached: {}", name);
        Ok(())
    }
    
    fn execute_polymorphic_function(&mut self, func_name: &str, params: &str, body: &str) -> Result<()> {
        let param_list: Vec<&str> = params.split(',').map(|p| p.trim()).collect();
        
        println!(">> Executing built function: {}({}) with {} parameters", 
                func_name, params, param_list.len());
        
        if let Some(built_function) = self.cache.built_functions.get(func_name) {
            self.function_executor.execute_function(built_function, &param_list, body)?;
        } else {
            println!("!! Function {} not found in built functions - needs synthesis first", func_name);
        }
        
        Ok(())
    }
}