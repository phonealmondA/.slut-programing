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
mod target_seeker;

use function_builder::FunctionBuilder;
use function_executor::FunctionExecutor;
use target_seeker::QuantumTargetSeeker;

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
    variables: HashMap<String, String>,
    quantum_states: HashMap<String, CollapsedState>,
    variable_attempts: HashMap<String, Vec<VariableAttempt>>,
    built_functions: HashMap<String, BuiltFunction>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("** Quantum Consciousness Observer (Rust Edition)");
    println!(">> Building programs from intentions, not instructions");
    println!(">> Target-seeking mathematics activated");
    println!(">> Executing: {:?}", args.file);
    
    let mut transpiler = QuantumTranspiler::new()?;
    
    for i in 1..=args.observations {
        if args.observations > 1 {
            println!("\n== OBSERVATION {} ==", i);
        }
        
        transpiler.execute_file(&args.file)?;
        
        if i < args.observations {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
    
    println!("** Quantum consciousness execution complete!");
    Ok(())
}

struct QuantumTranspiler {
    cache: QuantumCache,
    execution_count: u32,
    function_builder: FunctionBuilder,
    function_executor: FunctionExecutor,
    target_seeker: QuantumTargetSeeker,
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
            }
        });
        
        if !cache.templates.is_empty() || !cache.built_functions.is_empty() {
            println!("** Loaded previous quantum states and built functions from cache");
        }
        
        let function_builder = FunctionBuilder::new()?;
        let function_executor = FunctionExecutor::new()?;
        let target_seeker = QuantumTargetSeeker::new(cache.quantum_states.clone(), cache.variable_attempts.clone());
        
        Ok(Self {
            cache,
            execution_count: 0,
            function_builder,
            function_executor,
            target_seeker,
        })
    }
    
    fn load_cache() -> Result<QuantumCache> {
        let content = fs::read_to_string("quantum_consciousness_cache.json")?;
        let cache: QuantumCache = serde_json::from_str(&content)?;
        Ok(cache)
    }
    
    fn save_cache(&mut self) -> Result<()> {
        // Update cache with target seeker state
        self.cache.quantum_states = self.target_seeker.get_collapsed_states();
        self.cache.variable_attempts = self.target_seeker.get_variable_attempts();
        
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
        
        let main_regex = Regex::new(r"\*\s*<main>\s*(\w+)\s*\{[^}]*\^\s*observe_execution\s*\{([\s\S]*?)\}\s*\}")?;
        
        if let Some(captures) = main_regex.captures(source) {
            let class_name = &captures[1];
            let body = &captures[2];
            
            println!(">> Quantum consciousness activated for: {}", class_name);
            self.execute_main_body(body, class_name)?;
            println!("** Program built and executed successfully!");
        } else {
            println!("!! No main class found in source");
        }
        
        Ok(())
    }
    
    fn execute_main_body(&mut self, body: &str, class_name: &str) -> Result<()> {
        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") { continue; }
            
            self.execute_statement(line, class_name)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, statement: &str, class_name: &str) -> Result<()> {
        // Target-seeking mathematics: result([target]) <> randomChoice([inputs])
        let target_seeking_regex = Regex::new(r"(\w+)\s*\(\s*\[([^\]]+)\]\s*\)\s*<>\s*randomChoice\s*\(\s*\[([^\]]*)\]\s*\)")?;
        if let Some(captures) = target_seeking_regex.captures(statement) {
            let var_name = &captures[1];
            let target: f64 = captures[2].parse().unwrap_or(0.0);
            let inputs_str = &captures[3];
            
            // Parse inputs (numbers and function calls)
            let inputs: Vec<f64> = inputs_str.split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .filter(|&x| x != 0.0) // Remove invalid parses
                .collect();
            
            if inputs.is_empty() {
                println!("!! No valid numeric inputs found for target-seeking");
                return Ok(());
            }
            
            println!(">> Target-seeking mathematics: {} = {} from {:?}", var_name, target, inputs);
            
            let result = self.target_seeker.find_target_solution(target, &inputs, class_name, var_name)?;
            println!("== Quantum collapse: {} = {} (accuracy: {:.1}%)", var_name, result.result, result.accuracy);
            
            // Store result in cache
            let cache_key = format!("{}-{}-{}", class_name, target, inputs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
            self.cache.quantum_states.insert(cache_key, CollapsedState {
                result: result.result,
                equation: result.equation,
                accuracy: result.accuracy,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
                calculation_time: result.calculation_time,
            });
            
            return Ok(());
        }
        
        // Function synthesis
        let poly_synthesis_regex = Regex::new(r"(\w+)\s*\(\s*([^)]*)\s*\)\s*<>\s*function\s*\(\s*(\w+)\s*\)")?;
        if let Some(captures) = poly_synthesis_regex.captures(statement) {
            let func_name = &captures[1];
            let params = &captures[2];
            let func_type = &captures[3];
            return self.synthesize_polymorphic_function(func_name, params, func_type);
        }
        
        // Function execution
        let poly_exec_regex = Regex::new(r#"(\w+)\s*\(\s*([^)]+)\s*\)\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = poly_exec_regex.captures(statement) {
            let func_name = &captures[1];
            let params = &captures[2];
            let body = &captures[3];
            return self.execute_polymorphic_function(func_name, params, body);
        }
        
        // Speak statements with variable interpolation
        let speak_regex = Regex::new(r#"speak\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = speak_regex.captures(statement) {
            let message = &captures[1];
            // For now, just print the message as-is
            // TODO: Implement variable interpolation (~variable~)
            println!("{}", message);
            return Ok(());
        }
        
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