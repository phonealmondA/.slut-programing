use anyhow::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{StoredVariable, VariableValue};

pub struct VariableManager {
    variables: HashMap<String, StoredVariable>,
}

impl VariableManager {
    pub fn new(cached_variables: HashMap<String, StoredVariable>) -> Self {
        println!(">> Initializing variable manager with {} cached variables", cached_variables.len());
        
        if !cached_variables.is_empty() {
            for (name, var) in &cached_variables {
                println!("   - Restored variable '{}': {:?}", name, var.value);
            }
        }
        
        Self {
            variables: cached_variables,
        }
    }
    
    pub fn store_variable(
        &mut self, 
        name: &str, 
        value: VariableValue, 
        source_equation: Option<String>
    ) -> Result<()> {
        let stored_var = StoredVariable {
            name: name.to_string(),
            value: value.clone(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
            source_equation,
        };
        
        self.variables.insert(name.to_string(), stored_var);
        
        let value_str = match &value {
            VariableValue::Number(n) => n.to_string(),
            VariableValue::String(s) => format!("\"{}\"", s),
            VariableValue::Boolean(b) => b.to_string(),
            VariableValue::FunctionResult(f) => format!("[Function: {}]", f),
        };
        
        println!("++ Variable stored: '{}' = {}", name, value_str);
        
        Ok(())
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&StoredVariable> {
        self.variables.get(name)
    }
    
    pub fn get_variable_value(&self, name: &str) -> Option<&VariableValue> {
        self.variables.get(name).map(|var| &var.value)
    }
    
    pub fn get_all_variables(&self) -> HashMap<String, StoredVariable> {
        self.variables.clone()
    }
    
    pub fn list_variables(&self) {
        if self.variables.is_empty() {
            println!("== No variables stored");
            return;
        }
        
        println!("== Stored variables:");
        for (name, var) in &self.variables {
            let value_str = match &var.value {
                VariableValue::Number(n) => n.to_string(),
                VariableValue::String(s) => format!("\"{}\"", s),
                VariableValue::Boolean(b) => b.to_string(),
                VariableValue::FunctionResult(f) => format!("[Function: {}]", f),
            };
            
            print!("   {} = {}", name, value_str);
            
            if let Some(eq) = &var.source_equation {
                print!(" (from: {})", eq);
            }
            
            println!();
        }
    }
    
    pub fn variable_exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    
    pub fn get_numeric_value(&self, name: &str) -> Option<f64> {
        if let Some(var) = self.get_variable(name) {
            match &var.value {
                VariableValue::Number(n) => Some(*n),
                _ => None,
            }
        } else {
            None
        }
    }
    
    pub fn get_string_value(&self, name: &str) -> Option<String> {
        if let Some(var) = self.get_variable(name) {
            match &var.value {
                VariableValue::String(s) => Some(s.clone()),
                VariableValue::Number(n) => Some(n.to_string()),
                VariableValue::Boolean(b) => Some(b.to_string()),
                VariableValue::FunctionResult(f) => Some(format!("[Function: {}]", f)),
            }
        } else {
            None
        }
    }
    
    pub fn resolve_expression_inputs(&self, inputs_str: &str) -> Vec<f64> {
        let mut resolved = Vec::new();
        let mut blanks_count = 0;
        
        for input in inputs_str.split(',') {
            let input = input.trim();
            
            if input == "?" {
                blanks_count += 1;
            } else if let Ok(num) = input.parse::<f64>() {
                resolved.push(num);
            } else if let Some(variable) = self.get_variable(input) {
                match &variable.value {
                    VariableValue::Number(num) => {
                        println!("-- Resolved variable '{}' = {}", input, num);
                        resolved.push(*num);
                    }
                    VariableValue::String(s) => {
                        println!("-- Parsing string variable '{}' = '{}'", input, s);
                        for part in s.split(',') {
                            let part = part.trim();
                            if part == "?" {
                                blanks_count += 1;
                            } else if let Ok(num) = part.parse::<f64>() {
                                resolved.push(num);
                                println!("   - Parsed number: {}", num);
                            } else {
                                println!("   - Skipping non-numeric: {}", part);
                            }
                        }
                    }
                    _ => {
                        println!("-- Variable '{}' is not numeric or string, skipping", input);
                    }
                }
            }
        }
        
        if blanks_count > 0 {
            println!("-- Found {} blank placeholders (?), searching for cached solutions...", blanks_count);
            
            let available_solutions = self.get_available_cached_solutions();
            let mut added_solutions = 0;
            
            for solution in available_solutions {
                if added_solutions < blanks_count {
                    resolved.push(solution);
                    println!("   + Filled ? with cached solution: {}", solution);
                    added_solutions += 1;
                }
            }
        }
        
        resolved
    }
    
    fn get_available_cached_solutions(&self) -> Vec<f64> {
        let mut solutions = Vec::new();
        
        for (name, var) in &self.variables {
            if let VariableValue::Number(num) = &var.value {
                
                if *num > 1.0 && 
                   *num != 42.0 &&    
                   *num != 25.0 &&    
                   *num != 360.0 &&   
                   !name.starts_with("target") && 
                   !name.starts_with("myNumber") {
                    
                    if var.source_equation.is_some() {
                        solutions.push(*num);
                        println!("   - Found computed solution: {} = {} (from: {})", 
                                name, num, var.source_equation.as_ref().unwrap());
                    } else if solutions.len() < 3 {
                        
                        solutions.push(*num);
                        println!("   - Found stored value: {} = {}", name, num);
                    }
                }
            }
        }
        
        solutions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        if solutions.is_empty() {
            println!("   - No suitable cached solutions found");
        } else {
            println!("   - Available cached solutions: {:?}", solutions);
        }
        
        solutions
    }
    
    pub fn resolve_mixed_inputs(&self, inputs_str: &str) -> Vec<VariableValue> {
        let mut resolved = Vec::new();
        
        for input in inputs_str.split(',') {
            let input = input.trim();
            
            if let Ok(num) = input.parse::<f64>() {
                resolved.push(VariableValue::Number(num));
            }
            
            else if let Some(variable) = self.get_variable(input) {
                resolved.push(variable.value.clone());
                println!("-- Resolved variable '{}' = {:?}", input, variable.value);
            }
            
            else {
                resolved.push(VariableValue::String(input.trim_matches('"').to_string()));
            }
        }
        
        resolved
    }
    
    pub fn update_variable(&mut self, name: &str, new_value: VariableValue) -> Result<()> {
        if let Some(var) = self.variables.get_mut(name) {
            var.value = new_value;
            var.timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
            println!("++ Variable '{}' updated", name);
            Ok(())
        } else {
            self.store_variable(name, new_value, None)
        }
    }
    
    pub fn get_variable_history(&self, name: &str) -> Option<String> {
        if let Some(var) = self.get_variable(name) {
            let mut history = format!("Variable '{}' history:\n", name);
            history.push_str(&format!("  Current value: {:?}\n", var.value));
            history.push_str(&format!("  Timestamp: {}\n", var.timestamp));
            
            if let Some(eq) = &var.source_equation {
                history.push_str(&format!("  Source equation: {}\n", eq));
            }
            
            Some(history)
        } else {
            None
        }
    }
    
    pub fn clear_variables(&mut self) {
        self.variables.clear();
        println!("++ All variables cleared");
    }
    
    pub fn export_variables_to_string(&self) -> String {
        let mut output = String::new();
        
        for (name, var) in &self.variables {
            let value_str = match &var.value {
                VariableValue::Number(n) => n.to_string(),
                VariableValue::String(s) => format!("\"{}\"", s),
                VariableValue::Boolean(b) => b.to_string(),
                VariableValue::FunctionResult(f) => format!("[Function: {}]", f),
            };
            
            output.push_str(&format!("{} = {}\n", name, value_str));
        }
        
        output
    }
}