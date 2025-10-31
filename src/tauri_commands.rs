use tauri::State;
use std::sync::Mutex;
use std::path::PathBuf;
use anyhow::Result;

use crate::QuantumTranspiler;

/// Shared state that the UI can access
pub struct AppState {
    pub transpiler: Mutex<Option<QuantumTranspiler>>,
    pub is_running: Mutex<bool>,
    pub current_file: Mutex<Option<String>>,
    pub observation_count: Mutex<u32>,
    pub last_accuracy: Mutex<f64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            transpiler: Mutex::new(None),
            is_running: Mutex::new(false),
            current_file: Mutex::new(None),
            observation_count: Mutex::new(0),
            last_accuracy: Mutex::new(0.0),
        }
    }
}

/// Statistics returned to the frontend
#[derive(serde::Serialize, Default)]
pub struct CacheStats {
    pub variable_count: usize,
    pub solution_count: usize,
    pub last_accuracy: f64,
    pub observation_count: u32,
}

/// Command to run a .slut file once
#[tauri::command]
pub async fn run_file(file_path: String, state: State<'_, AppState>) -> Result<String, String> {
    let mut transpiler_guard = state.transpiler.lock().unwrap();

    // Initialize transpiler if needed
    if transpiler_guard.is_none() {
        match QuantumTranspiler::new() {
            Ok(trans) => *transpiler_guard = Some(trans),
            Err(e) => return Err(format!("Failed to initialize transpiler: {}", e)),
        }
    }

    let transpiler = transpiler_guard.as_mut().unwrap();

    // Execute the file
    match transpiler.execute_file(&PathBuf::from(&file_path)) {
        Ok(_) => {
            // Increment observation count
            let mut obs_count = state.observation_count.lock().unwrap();
            *obs_count += 1;

            // Update current file
            let mut current_file = state.current_file.lock().unwrap();
            *current_file = Some(file_path.clone());

            Ok(format!("Execution complete for observation {}", *obs_count))
        }
        Err(e) => Err(format!("Execution error: {}", e)),
    }
}

/// Command to get current cache stats
#[tauri::command]
pub fn get_cache_stats(state: State<'_, AppState>) -> Result<CacheStats, String> {
    let transpiler_guard = state.transpiler.lock().unwrap();

    if let Some(transpiler) = transpiler_guard.as_ref() {
        let variable_count = transpiler.variable_manager.get_all_variables().len();
        let solution_count = transpiler.math_engine.get_solutions().len();
        let last_accuracy = *state.last_accuracy.lock().unwrap();
        let observation_count = *state.observation_count.lock().unwrap();

        Ok(CacheStats {
            variable_count,
            solution_count,
            last_accuracy,
            observation_count,
        })
    } else {
        Ok(CacheStats::default())
    }
}

/// Command to run until solved (with max attempts)
#[tauri::command]
pub async fn run_until_solved(
    file_path: String,
    max_attempts: u32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Set running flag
    *state.is_running.lock().unwrap() = true;

    let mut attempts = 0;
    let mut best_accuracy = 0.0;

    while attempts < max_attempts && best_accuracy < 100.0 {
        // Check if user stopped it
        if !*state.is_running.lock().unwrap() {
            return Ok(format!("Stopped after {} attempts", attempts));
        }

        // Run the file
        match run_file(file_path.clone(), state.clone()).await {
            Ok(msg) => {
                println!("{}", msg);
            }
            Err(e) => {
                *state.is_running.lock().unwrap() = false;
                return Err(e);
            }
        }

        // Get updated stats
        let stats = get_cache_stats(state.clone()).map_err(|e| e.to_string())?;
        best_accuracy = stats.last_accuracy;

        // Update accuracy in state
        *state.last_accuracy.lock().unwrap() = best_accuracy;

        attempts += 1;

        // Small delay between attempts
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    // Clear running flag
    *state.is_running.lock().unwrap() = false;

    Ok(format!("Completed {} attempts. Best accuracy: {:.1}%", attempts, best_accuracy))
}

/// Command to stop running execution
#[tauri::command]
pub fn stop_execution(state: State<'_, AppState>) -> Result<(), String> {
    *state.is_running.lock().unwrap() = false;
    Ok(())
}

/// Command to reset the transpiler state
#[tauri::command]
pub fn reset_transpiler(state: State<'_, AppState>) -> Result<(), String> {
    let mut transpiler_guard = state.transpiler.lock().unwrap();
    *transpiler_guard = None;

    *state.observation_count.lock().unwrap() = 0;
    *state.last_accuracy.lock().unwrap() = 0.0;
    *state.current_file.lock().unwrap() = None;
    *state.is_running.lock().unwrap() = false;

    Ok(())
}
