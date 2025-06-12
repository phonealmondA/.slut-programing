use anyhow::Result;
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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
struct FunctionCache {
    templates: HashMap<String, CachedTemplate>,
    variables: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedTemplate {
    name: String,
    func_type: String,
    timestamp: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("** Quantum Consciousness Observer (Rust Edition)");
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
    cache: FunctionCache,
    execution_count: u32,
}

impl QuantumTranspiler {
    fn new() -> Result<Self> {
        let cache = Self::load_cache().unwrap_or_else(|_| {
            println!("** Starting with fresh function synthesis cache");
            FunctionCache {
                templates: HashMap::new(),
                variables: HashMap::new(),
            }
        });
        
        if !cache.templates.is_empty() || !cache.variables.is_empty() {
            println!("** Loaded previous function templates and variables from cache");
        }
        
        Ok(Self {
            cache,
            execution_count: 0,
        })
    }
    
    fn load_cache() -> Result<FunctionCache> {
        let content = fs::read_to_string("function_synthesis_cache.json")?;
        let cache: FunctionCache = serde_json::from_str(&content)?;
        Ok(cache)
    }
    
    fn save_cache(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.cache)?;
        fs::write("function_synthesis_cache.json", content)?;
        Ok(())
    }
    
    fn execute_file(&mut self, file_path: &PathBuf) -> Result<()> {
        let source = fs::read_to_string(file_path)?;
        self.parse_and_execute(&source)?;
        self.save_cache()?;
        Ok(())
    }
    
    fn parse_and_execute(&mut self, source: &str) -> Result<()> {
        println!(">> Variable Assignment Function Synthesis activated...");
        println!("** Session started at: {}", 
                 SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        
        // Extract main class
        let main_regex = Regex::new(r"\*\s*<main>\s*(\w+)\s*\{[^}]*\^\s*observe_execution\s*\{([\s\S]*?)\}\s*\}")?;
        
        if let Some(captures) = main_regex.captures(source) {
            let _class_name = &captures[1];
            let body = &captures[2];
            
            println!(">> Initiating variable assignment function synthesis...");
            self.execute_main_body(body)?;
            println!("** Function synthesis complete!");
        } else {
            println!("!! No main class found in source");
        }
        
        Ok(())
    }
    
    fn execute_main_body(&mut self, body: &str) -> Result<()> {
        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            
            self.execute_statement(line)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, statement: &str) -> Result<()> {
        // Function synthesis: testA(a) <> function(while)
        let func_synthesis_regex = Regex::new(r"(\w+)\s*\(\s*\w*\s*\)\s*<>\s*function\s*\(\s*(\w+)\s*\)")?;
        if let Some(captures) = func_synthesis_regex.captures(statement) {
            let func_name = &captures[1];
            let func_type = &captures[2];
            return self.synthesize_function(func_name, func_type);
        }
        
        // Variable assignment: tA <> testB(2)("console.log('hello')")
        let var_assign_regex = Regex::new(r#"(\w+)\s*<>\s*(\w+)\s*\(\s*(\d+)\s*\)\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = var_assign_regex.captures(statement) {
            let var_name = &captures[1];
            let func_name = &captures[2];
            let limit: u32 = captures[3].parse()?;
            let body = &captures[4];
            return self.execute_and_store(var_name, func_name, limit, body);
        }
        
        // Function template assignment: another <> testA()
        let template_assign_regex = Regex::new(r"(\w+)\s*<>\s*(\w+)\s*\(\s*\)")?;
        if let Some(captures) = template_assign_regex.captures(statement) {
            let var_name = &captures[1];
            let func_name = &captures[2];
            return self.store_template(var_name, func_name);
        }
        
        // Function execution with variable: testA(3)(tA)
        let func_exec_regex = Regex::new(r"(\w+)\s*\(\s*(\d+)\s*\)\s*\(\s*(\w+)\s*\)")?;
        if let Some(captures) = func_exec_regex.captures(statement) {
            let func_name = &captures[1];
            let limit: u32 = captures[2].parse()?;
            let var_name = &captures[3];
            return self.execute_with_variable(func_name, limit, var_name);
        }
        
        // Simple function execution: testA(5)("hello")
        let simple_exec_regex = Regex::new(r#"(\w+)\s*\(\s*(\d+)\s*\)\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = simple_exec_regex.captures(statement) {
            let func_name = &captures[1];
            let limit: u32 = captures[2].parse()?;
            let body = &captures[3];
            return self.execute_function(func_name, limit, body);
        }
        
        // Speak statements: speak("message")
        let speak_regex = Regex::new(r#"speak\s*\(\s*"([^"]*)"\s*\)"#)?;
        if let Some(captures) = speak_regex.captures(statement) {
            println!("{}", &captures[1]);
            return Ok(());
        }
        
        // Woof statements: woof variable
        let woof_regex = Regex::new(r"woof\s+(\w+)")?;
        if let Some(captures) = woof_regex.captures(statement) {
            let var_name = &captures[1];
            return self.execute_variable(var_name);
        }
        
        Ok(())
    }
    
    fn synthesize_function(&mut self, name: &str, func_type: &str) -> Result<()> {
        let cache_key = format!("{}_{}", name, func_type);
        
        println!(">> Synthesizing function: {} with type: {}", name, func_type);
        
        if self.cache.templates.contains_key(&cache_key) {
            println!("== Using cached function template: {}", name);
            return Ok(());
        }
        
        let template = CachedTemplate {
            name: name.to_string(),
            func_type: func_type.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
        };
        
        self.cache.templates.insert(cache_key, template);
        println!("** Function template synthesized and cached: {}", name);
        
        Ok(())
    }
    
    fn execute_and_store(&mut self, var_name: &str, func_name: &str, limit: u32, body: &str) -> Result<()> {
        let result = self.execute_function_internal(func_name, limit, body)?;
        
        println!("-- Storing variable: {} = {}", var_name, result);
        self.cache.variables.insert(var_name.to_string(), result);
        
        Ok(())
    }
    
    fn store_template(&mut self, var_name: &str, _func_name: &str) -> Result<()> {
        println!("-- Storing variable: {} = [Function]", var_name);
        self.cache.variables.insert(var_name.to_string(), "[Function]".to_string());
        Ok(())
    }
    
    fn execute_with_variable(&mut self, func_name: &str, limit: u32, var_name: &str) -> Result<()> {
        // Clone the variable value to avoid borrowing issues
        if let Some(variable_value) = self.cache.variables.get(var_name).cloned() {
            println!("-- Retrieved variable: {}", var_name);
            println!(">> Executing {}({}) with variable: {}", func_name, limit, var_name);
            self.execute_function_internal(func_name, limit, &variable_value)?;
        } else {
            println!("!! Cannot execute {} - variable {} not found", func_name, var_name);
        }
        Ok(())
    }
    
    fn execute_function(&mut self, func_name: &str, limit: u32, body: &str) -> Result<()> {
        self.execute_function_internal(func_name, limit, body)?;
        Ok(())
    }
    
    fn execute_function_internal(&mut self, func_name: &str, limit: u32, body: &str) -> Result<String> {
        self.execution_count += 1;
        
        println!(">> Execution #{} - Running function: {}({})", 
                self.execution_count, func_name, limit);
        
        println!("-- Executing {} while loop {} times with 1 bodies", func_name, limit);
        
        for i in 1..=limit {
            println!("Loop iteration {}:", i);
            println!("  Body 1:");
            self.execute_body(body)?;
        }
        
        let result = format!("{} completed {} times", func_name, limit);
        println!("== Function execution complete: {}", result);
        Ok(result)
    }
    
    fn execute_body(&self, body: &str) -> Result<()> {
        if body.starts_with("console.log") {
            // Extract message from console.log('message')
            if let Some(start) = body.find('\'') {
                if let Some(end) = body.rfind('\'') {
                    let message = &body[start + 1..end];
                    println!("    {}", message);
                    return Ok(());
                }
            }
        }
        
        // Fallback: print the body as-is
        println!("    {}", body);
        Ok(())
    }
    
    fn execute_variable(&self, var_name: &str) -> Result<()> {
        if let Some(variable_value) = self.cache.variables.get(var_name) {
            println!("-- Retrieved variable: {}", var_name);
            if variable_value == "[Function]" {
                println!("Executing stored function template");
            } else {
                println!("Final result: {}", variable_value);
            }
        } else {
            println!("!! Variable not found: {}", var_name);
        }
        Ok(())
    }
}