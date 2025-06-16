use anyhow::Result;
use crate::BuiltFunction;

pub struct FunctionExecutor {
    // Simple statement executor
}

impl FunctionExecutor {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    pub fn execute_function(&self, built_function: &BuiltFunction, params: &[&str], body: &str) -> Result<()> {
        println!(">> Loading built function: {}", built_function.name);
        
        // Find the right variant based on parameter count
        let param_count = params.len();
        
        if let Some(variant) = built_function.variants.iter().find(|v| v.parameter_count == param_count) {
            println!("== Using variant: {} ({})", variant.rust_function_name, variant.parameter_pattern);
            
            // Execute the actual function (no more simulation!)
            match variant.parameter_pattern.as_str() {
                "count" => {
                    if let Ok(count) = params[0].parse::<u32>() {
                        self.execute_count_loop(count, body)?;
                    }
                }
                "range" => {
                    if params.len() >= 2 {
                        let start: u32 = params[0].parse().unwrap_or(0);
                        let end: u32 = params[1].parse().unwrap_or(0);
                        self.execute_range_loop(start, end, body)?;
                    }
                }
                "step" => {
                    if params.len() >= 3 {
                        let start: u32 = params[0].parse().unwrap_or(0);
                        let end: u32 = params[1].parse().unwrap_or(0);
                        let step: u32 = params[2].parse().unwrap_or(1);
                        self.execute_step_loop(start, end, step, body)?;
                    }
                }
                _ => {
                    println!("!! Unknown pattern: {}", variant.parameter_pattern);
                }
            }
            
            println!("== Function execution complete: {}", built_function.name);
        } else {
            println!("!! No variant found for {} parameters in function {}", 
                    param_count, built_function.name);
        }
        
        Ok(())
    }
    
    // These now actually execute the code instead of simulating
    fn execute_count_loop(&self, count: u32, body: &str) -> Result<()> {
        println!("-- Executing REAL count-based loop: {} iterations", count);
        for i in 0..count {
            self.execute_statement(body, i)?;
        }
        Ok(())
    }
    
    fn execute_range_loop(&self, start: u32, end: u32, body: &str) -> Result<()> {
        println!("-- Executing REAL range-based loop: {} to {}", start, end);
        for i in start..end {
            self.execute_statement(body, i)?;
        }
        Ok(())
    }
    
    fn execute_step_loop(&self, start: u32, end: u32, step: u32, body: &str) -> Result<()> {
        println!("-- Executing REAL step-based loop: {} to {} by {}", start, end, step);
        let mut i = start;
        while i < end {
            self.execute_statement(body, i)?;
            i += step;
        }
        Ok(())
    }
    
    // Basic statement execution - starts simple, can be expanded
    fn execute_statement(&self, statement: &str, iteration: u32) -> Result<()> {
        let statement = statement.trim();
        
        // Handle println! statements
        if statement.starts_with("println!") {
            self.execute_println(statement, iteration)?;
        }
        // Future: add more statement types here (variable assignments, function calls, etc.)
        else {
            println!("!! Unknown statement type: {}", statement);
        }
        
        Ok(())
    }
    
    fn execute_println(&self, statement: &str, iteration: u32) -> Result<()> {
        // Parse println!("text") or println!("text {}", variable)
        if let Some(content) = self.extract_println_content(statement) {
            // Handle simple substitutions
            let output = content
                .replace("{}", &iteration.to_string())
                .replace("{i}", &iteration.to_string());
            
            println!("{}", output);
        } else {
            println!("!! Could not parse println statement: {}", statement);
        }
        
        Ok(())
    }
    
    fn extract_println_content(&self, statement: &str) -> Option<String> {
        // Extract content from println!("...") or println!(\"...\")
        if statement.starts_with("println!(\"") && statement.ends_with("\")") {
            let content = &statement[9..statement.len()-2]; // Remove println!(" and ")
            // Unescape the content
            let unescaped = content.replace("\\\"", "\"");
            Some(unescaped)
        } else if statement.starts_with("println!(\\\"") && statement.ends_with("\\\")") {
            let content = &statement[11..statement.len()-3]; // Remove println!(\" and \")
            Some(content.to_string())
        } else {
            None
        }
    }
}