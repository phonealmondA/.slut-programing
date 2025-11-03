use anyhow::Result;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{StoredVariable, VariableValue, ConsoleCallback};

pub struct VariableManager {
    variables: HashMap<String, StoredVariable>,
    console_callback: Option<ConsoleCallback>,
}

impl VariableManager {
    pub fn new(cached_variables: HashMap<String, StoredVariable>) -> Self {
        let msg = format!(">> Initializing variable manager with {} cached variables", cached_variables.len());
        println!("{}", msg);

        if !cached_variables.is_empty() {
            for (name, var) in &cached_variables {
                let var_msg = format!("   - Restored variable '{}': {:?}", name, var.value);
                println!("{}", var_msg);
            }
        }

        Self {
            variables: cached_variables,
            console_callback: None,
        }
    }

    pub fn set_console_callback(&mut self, callback: ConsoleCallback) {
        self.console_callback = Some(callback);
    }

    fn emit(&self, message: String, level: &str) {
        println!("{}", message);
        if let Some(callback) = &self.console_callback {
            callback(message, level);
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

        self.emit(format!("++ Variable stored: '{}' = {}", name, value_str), "info");

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
        self.resolve_expression_inputs_with_target(inputs_str, None)
    }

    pub fn resolve_expression_inputs_with_target(&self, inputs_str: &str, target: Option<f64>) -> Vec<f64> {
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
            self.emit(format!("-- Found {} blank placeholders (?), searching for cached solutions...", blanks_count), "info");
            if let Some(t) = target {
                self.emit(format!("   >> Target-aware selection enabled for target: {}", t), "info");
            }

            let available_solutions = self.get_available_cached_solutions();

            // Use diverse selection strategy to avoid filling all blanks with same value
            let selected_solutions = self.select_diverse_solutions(&available_solutions, blanks_count, target);

            let filled_count = selected_solutions.len();
            for solution in selected_solutions {
                resolved.push(solution);
                self.emit(format!("   + Filled ? with cached solution: {}", solution), "info");
            }

            // Warn if we couldn't fill all blanks
            if filled_count < blanks_count {
                println!("   >> Warning: Need {} blanks but only have {} cached values",
                         blanks_count, filled_count);
                println!("   >> Some placeholders remain unfilled. Provide more concrete inputs or build up cache.");
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
                        self.emit(format!("   - Found computed solution: {} = {} (from: {})",
                                name, num, var.source_equation.as_ref().unwrap()), "info");
                    } else if solutions.len() < 3 {

                        solutions.push(*num);
                        self.emit(format!("   - Found stored value: {} = {}", name, num), "info");
                    }
                }
            }
        }

        // Remove duplicates to ensure diversity
        solutions.sort_by(|a, b| a.partial_cmp(b).unwrap());
        solutions.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);

        if solutions.is_empty() {
            self.emit("   - No suitable cached solutions found".to_string(), "info");
        } else {
            self.emit(format!("   - Available cached solutions (deduplicated): {:?}", solutions), "info");
        }

        solutions
    }

    /// Selects diverse cached solutions for placeholder filling
    /// Strategy: Distribute values across small, medium, and large ranges
    /// With optional target-aware optimization
    fn select_diverse_solutions(&self, available: &[f64], count: usize, target: Option<f64>) -> Vec<f64> {
        if available.is_empty() {
            return Vec::new();
        }

        if available.len() <= count {
            // If we have fewer or equal cached values than needed, return all
            return available.to_vec();
        }

        let mut selected = Vec::new();
        let len = available.len();

        self.emit(format!("   >> Selecting {} diverse values from {} available solutions", count, len), "info");

        // Target-aware selection: Filter and prioritize based on target size
        if let Some(t) = target {
            let filtered = self.filter_by_target_range(available, t);

            if !filtered.is_empty() && filtered.len() >= count {
                // Use filtered set for better target alignment
                self.emit(format!("      >> Using target-aware filtered set of {} values", filtered.len()), "info");
                return self.distribute_values(&filtered, count);
            } else if !filtered.is_empty() {
                // Use what we have from filtered set, then add from full set
                self.emit(format!("      >> Target filtering gave {} values, need {}", filtered.len(), count), "info");
                selected.extend_from_slice(&filtered);
                let remaining = count - selected.len();

                // Add diverse values from remaining set
                let remaining_values: Vec<f64> = available.iter()
                    .filter(|v| !selected.contains(v))
                    .copied()
                    .collect();

                if !remaining_values.is_empty() {
                    let extra = self.distribute_values(&remaining_values, remaining);
                    selected.extend(extra);
                }

                return selected;
            }
            // If filtered is empty, fall through to default distribution
        }

        // Default diverse selection without target awareness
        self.distribute_values(available, count)
    }

    /// Filter cached values based on target size
    /// Strategy from improvement plan:
    /// - Small target (< 100): Prefer small values
    /// - Medium target (100-1000): Mix of small and large
    /// - Large target (> 1000): Prefer large values + small multipliers
    fn filter_by_target_range(&self, available: &[f64], target: f64) -> Vec<f64> {
        let mut filtered: Vec<f64> = if target < 100.0 {
            // Small target: prefer small values (< target * 2)
            self.emit(format!("      >> Small target ({}) - preferring small cached values", target), "info");
            available.iter()
                .filter(|&&v| v < target * 2.0)
                .copied()
                .collect()
        } else if target < 1000.0 {
            // Medium target: mix of small and large
            // Include values from 1 to target * 2
            self.emit(format!("      >> Medium target ({}) - selecting balanced range", target), "info");
            available.iter()
                .filter(|&&v| v >= 2.0 && v <= target * 2.0)
                .copied()
                .collect()
        } else {
            // Large target: prefer large values and small multipliers
            self.emit(format!("      >> Large target ({}) - preferring large values and small multipliers", target), "info");
            let small_multipliers: Vec<f64> = available.iter()
                .filter(|&&v| v >= 2.0 && v <= 20.0)
                .copied()
                .collect();

            let large_bases: Vec<f64> = available.iter()
                .filter(|&&v| v > 20.0 && v <= target * 1.5)
                .copied()
                .collect();

            // Combine small multipliers and large bases
            let mut combined = small_multipliers;
            combined.extend(large_bases);
            combined.sort_by(|a, b| a.partial_cmp(b).unwrap());
            combined.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
            combined
        };

        // Always ensure we have at least some values
        if filtered.is_empty() && !available.is_empty() {
            // Fallback: use values closest to target/3 (useful for multiplication)
            filtered = available.to_vec();
        }

        filtered
    }

    /// Distribute count values evenly across the available range
    fn distribute_values(&self, available: &[f64], count: usize) -> Vec<f64> {
        let mut selected = Vec::new();
        let len = available.len();

        if len <= count {
            return available.to_vec();
        }

        match count {
            1 => {
                // For single selection, pick the largest value to maximize potential
                // This helps build bigger numbers through multiplication/exponentiation
                let idx = len - 1; // Pick the largest available value
                selected.push(available[idx]);
                self.emit(format!("      + Selected largest value to maximize potential: {}", available[idx]), "info");
            }
            2 => {
                // Take smallest and largest for maximum diversity
                selected.push(available[0]);
                selected.push(available[len - 1]);
                println!("      + Selected smallest: {}", available[0]);
                println!("      + Selected largest: {}", available[len - 1]);
            }
            _ => {
                // Take smallest, evenly distributed middle values, and largest
                selected.push(available[0]);
                println!("      + Selected smallest: {}", available[0]);

                // Calculate how many middle values we need
                let middle_count = count - 2;
                let step = (len - 2) as f64 / (middle_count + 1) as f64;

                for i in 1..=middle_count {
                    let idx = (i as f64 * step).round() as usize;
                    let idx = idx.min(len - 2).max(1); // Ensure valid range
                    selected.push(available[idx]);
                    println!("      + Selected middle value: {}", available[idx]);
                }

                selected.push(available[len - 1]);
                println!("      + Selected largest: {}", available[len - 1]);
            }
        }

        selected
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