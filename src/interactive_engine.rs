// src/interactive_engine.rs - Interactive Mathematical Reasoning Engine

use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{VariableValue, MathSolution, VariableAttempt};
use crate::math_engine::MathEngine;
use crate::variable_manager::VariableManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveSession {
    session_id: String,
    learned_solutions: HashMap<String, CachedSolution>,
    user_inputs: Vec<UserInteraction>,
    total_problems_solved: u32,
    session_start_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedSolution {
    pub target: f64,
    pub inputs: Vec<f64>,
    pub equation: String,
    pub result: f64,
    pub timestamp: u64,
    pub success_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInteraction {
    target: f64,
    provided_inputs: Vec<f64>,
    solution_found: String,
    result: f64,
    timestamp: u64,
    thinking_steps: Vec<String>,
}

pub enum UserInput {
    Problem { target: f64, inputs: Vec<f64> },
    Help,
    Stats,
    Quit,
}

pub struct InteractiveEngine {
    session: InteractiveSession,
    math_engine: MathEngine,
    variable_manager: VariableManager,
    session_file: String,
}

impl InteractiveEngine {
    pub fn new() -> Result<Self> {
        let session_file = "test_interactive/user_session_cache.json".to_string();
        
        // Create directory if it doesn't exist
        fs::create_dir_all("test_interactive")?;
        
        let session = Self::load_or_create_session(&session_file)?;
        
        // Load previous solutions into math engine
        let math_solutions = Self::convert_cached_to_math_solutions(&session.learned_solutions);
        let variable_attempts: HashMap<String, Vec<VariableAttempt>> = HashMap::new();
        
        let math_engine = MathEngine::new(math_solutions, variable_attempts);
        let variables: HashMap<String, crate::StoredVariable> = HashMap::new();
        let variable_manager = VariableManager::new(variables);
        
        println!("** Interactive Mathematical Reasoning Engine Initialized **");
        println!("** Loaded {} previous solutions from cache **", session.learned_solutions.len());
        
        Ok(Self {
            session,
            math_engine,
            variable_manager,
            session_file,
        })
    }
    
    fn load_or_create_session(file_path: &str) -> Result<InteractiveSession> {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                let session: InteractiveSession = serde_json::from_str(&content)?;
                println!("** Loaded previous session with {} learned solutions", session.learned_solutions.len());
                Ok(session)
            },
            Err(_) => {
                println!("** Creating new interactive session");
                Ok(InteractiveSession {
                    session_id: format!("session_{}", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()),
                    learned_solutions: HashMap::new(),
                    user_inputs: Vec::new(),
                    total_problems_solved: 0,
                    session_start_time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
                })
            }
        }
    }
    
    fn convert_cached_to_math_solutions(cached: &HashMap<String, CachedSolution>) -> HashMap<String, MathSolution> {
        let mut math_solutions = HashMap::new();
        
        for (key, cached_solution) in cached {
            let math_solution = MathSolution {
                result: cached_solution.result,
                equation: cached_solution.equation.clone(),
                accuracy: 100.0, // Cached solutions are exact
                timestamp: cached_solution.timestamp,
                attempts: cached_solution.success_count,
            };
            math_solutions.insert(key.clone(), math_solution);
        }
        
        math_solutions
    }
    
    pub fn run_interactive_session(&mut self) -> Result<()> {
        println!("\n=== QUANTUM INTERACTIVE MATHEMATICAL REASONING ===");
        println!("This system learns from each problem and uses previous solutions as building blocks.");
        println!("Type 'quit' or 'exit' to stop, 'help' for instructions.\n");
        
        loop {
            match self.get_user_problem()? {
                UserInput::Problem { target, inputs } => {
                    self.solve_interactive_problem(target, inputs)?;
                },
                UserInput::Help => {
                    self.show_help();
                },
                UserInput::Stats => {
                    self.show_statistics();
                },
                UserInput::Quit => {
                    break;
                },
            }
            
            println!();
        }
        
        self.save_session()?;
        println!("** Session saved. Thank you for using Quantum Consciousness! **");
        Ok(())
    }
    
    fn get_user_problem(&self) -> Result<UserInput> {
        print!("Enter target number (or 'help', 'stats', 'quit'): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        
        match input.as_str() {
            "quit" | "exit" => return Ok(UserInput::Quit),
            "help" => return Ok(UserInput::Help),
            "stats" => return Ok(UserInput::Stats),
            _ => {}
        }
        
        let target: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("!! Please enter a valid number");
                return self.get_user_problem();
            }
        };
        
        print!("Enter available numbers (comma-separated, use ? for blanks): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        let inputs = self.parse_user_inputs(input)?;
        
        Ok(UserInput::Problem { target, inputs })
    }
    
    fn parse_user_inputs(&self, input: &str) -> Result<Vec<f64>> {
        let mut numbers = Vec::new();
        
        for part in input.split(',') {
            let part = part.trim();
            if part == "?" {
                // Blank - we'll try to fill this from cached solutions
                continue;
            } else if let Ok(num) = part.parse::<f64>() {
                numbers.push(num);
            } else {
                println!("!! Skipping invalid input: {}", part);
            }
        }
        
        Ok(numbers)
    }
    
    fn solve_interactive_problem(&mut self, target: f64, mut inputs: Vec<f64>) -> Result<()> {
        println!("\n>> Analyzing problem: Find {} using {:?}", target, inputs);
        
        let mut thinking_steps = Vec::new();
        let start_time = std::time::Instant::now();
        
        // First try with provided inputs only
        thinking_steps.push(format!("Trying with provided inputs: {:?}", inputs));
        let mut solution = self.math_engine.solve_target(target, &inputs, "interactive", "interactive")?;
        
        if solution.accuracy < 100.0 {
            thinking_steps.push("No exact solution with provided inputs. Checking cached solutions...".to_string());
            
            // Try incorporating cached solutions
            let enhanced_inputs = self.enhance_inputs_with_cache(&inputs, target);
            if enhanced_inputs.len() > inputs.len() {
                thinking_steps.push(format!("Enhanced inputs with cached solutions: {:?}", enhanced_inputs));
                solution = self.math_engine.solve_target(target, &enhanced_inputs, "interactive", "interactive")?;
                inputs = enhanced_inputs; // Update inputs for caching
            }
        }
        
        let solve_time = start_time.elapsed();
        
        // Display thinking process
        println!("\n== THINKING PROCESS:");
        for (i, step) in thinking_steps.iter().enumerate() {
            println!("   {}. {}", i + 1, step);
        }
        
        // Display result
        if solution.accuracy >= 100.0 {
            println!("\n== SOLUTION FOUND!");
            println!("   Equation: {}", solution.equation);
            println!("   Result: {}", solution.result);
            println!("   Solve time: {:?}", solve_time);
            
            // Cache this solution for future use
            self.cache_solution(target, &inputs, &solution)?;
            
        } else {
            println!("\n!! NO EXACT SOLUTION FOUND");
            println!("   Best approximation: {} = {}", solution.equation, solution.result);
            println!("   Accuracy: {:.1}%", solution.accuracy);
        }
        
        // Record interaction
        let interaction = UserInteraction {
            target,
            provided_inputs: inputs,
            solution_found: solution.equation,
            result: solution.result,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
            thinking_steps,
        };
        
        self.session.user_inputs.push(interaction);
        self.session.total_problems_solved += 1;
        
        Ok(())
    }
    
    fn enhance_inputs_with_cache(&self, inputs: &[f64], target: f64) -> Vec<f64> {
        let mut enhanced = inputs.to_vec();
        
        // Look for cached solutions that could be useful
        for cached_solution in self.session.learned_solutions.values() {
            // Add cached results as potential inputs if they're not too large compared to target
            if cached_solution.result <= target * 2.0 && cached_solution.result >= 1.0 {
                if !enhanced.contains(&cached_solution.result) {
                    enhanced.push(cached_solution.result);
                    println!("   -- Adding cached solution {} (from: {})", 
                            cached_solution.result, cached_solution.equation);
                }
            }
            
            // Also add the original inputs from cached solutions if they're small
            for &input in &cached_solution.inputs {
                if input <= target && input >= 1.0 && !enhanced.contains(&input) {
                    enhanced.push(input);
                }
            }
        }
        
        enhanced
    }
    
    fn cache_solution(&mut self, target: f64, inputs: &[f64], solution: &MathSolution) -> Result<()> {
        let cache_key = format!("{}_{:?}", target, inputs);
        
        let cached_solution = CachedSolution {
            target,
            inputs: inputs.to_vec(),
            equation: solution.equation.clone(),
            result: solution.result,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
            success_count: 1,
        };
        
        // Update or insert solution
        if let Some(existing) = self.session.learned_solutions.get_mut(&cache_key) {
            existing.success_count += 1;
        } else {
            self.session.learned_solutions.insert(cache_key, cached_solution);
            println!("   ++ Solution cached for future use! (Total cached: {})", 
                    self.session.learned_solutions.len());
        }
        
        Ok(())
    }
    
    fn show_help(&self) {
        println!("\n=== HELP ===");
        println!("How to use:");
        println!("   - Enter a target number you want to reach");
        println!("   - Enter available numbers separated by commas");
        println!("   - Use '?' for blank spots (system will try to fill with cached solutions)");
        println!("");
        println!("Examples:");
        println!("   Target: 24, Numbers: 3,4,2  ->  System finds: 3 * 4 * 2 = 24");
        println!("   Target: 48, Numbers: 2,?    ->  System uses cached 24: 2 * 24 = 48");
        println!("   Target: 100, Numbers: ?     ->  System finds combinations from cache");
    }
    
    fn show_statistics(&self) {
        println!("\n=== STATISTICS ===");
        println!("Session ID: {}", self.session.session_id);
        println!("Problems solved this session: {}", self.session.total_problems_solved);
        println!("Total cached solutions: {}", self.session.learned_solutions.len());
        println!("Session duration: {} minutes", 
                (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 
                 - self.session.session_start_time) / 60000);
        
        if !self.session.learned_solutions.is_empty() {
            println!("\nRecent cached solutions:");
            let mut solutions: Vec<_> = self.session.learned_solutions.values().collect();
            solutions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            
            for (i, solution) in solutions.iter().take(5).enumerate() {
                println!("   {}. {} = {} (from: {:?})", 
                        i + 1, solution.equation, solution.result, solution.inputs);
            }
        }
    }
    
    fn save_session(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.session)?;
        fs::write(&self.session_file, content)?;
        println!("** Session data saved to: {}", self.session_file);
        Ok(())
    }
    
    pub fn get_session_stats(&self) -> (u32, usize) {
        (self.session.total_problems_solved, self.session.learned_solutions.len())
    }
}