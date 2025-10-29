use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::{BuiltFunction, FunctionVariant};

pub struct FunctionBuilder {
    functions_dir: String,
}

impl FunctionBuilder {
    pub fn new() -> Result<Self> {
        let functions_dir = "functions".to_string();
        
        fs::create_dir_all(&functions_dir)?;
        fs::create_dir_all(format!("{}/src", &functions_dir))?;
        
        let cargo_toml_path = format!("{}/Cargo.toml", &functions_dir);
        if !Path::new(&cargo_toml_path).exists() {
            let cargo_toml_content = r#"[package]
name = "quantum_functions"
version = "0.1.0"
edition = "2021"

[lib]
name = "quantum_functions"
crate-type = ["cdylib", "rlib"]

[dependencies]
"#;
            fs::write(&cargo_toml_path, cargo_toml_content)?;
            println!("** Created functions library Cargo.toml");
        }
        
        let lib_rs_path = format!("{}/src/lib.rs", &functions_dir);
        if !Path::new(&lib_rs_path).exists() {
            let lib_rs_content = r#"pub mod smart_loop;

pub use smart_loop::*;
"#;
            fs::write(&lib_rs_path, lib_rs_content)?;
            println!("** Created functions library lib.rs");
        }
        
        Ok(Self {
            functions_dir,
        })
    }
    
    pub fn build_function(&self, name: &str, func_type: &str, _param_count: usize) -> Result<BuiltFunction> {
        match func_type {
            "loop" => self.build_loop_function(name),
            "conditional" => self.build_conditional_function(name),
            _ => Err(anyhow::anyhow!("Unknown function type: {}", func_type)),
        }
    }
    
    fn build_loop_function(&self, name: &str) -> Result<BuiltFunction> {
        let file_name = format!("{}.rs", name.to_lowercase());
        let file_path = format!("{}/src/{}", self.functions_dir, file_name);
        
        println!(">> Building loop function variants for: {}", name);
        
        let rust_code = self.generate_loop_code(name)?;
        
        fs::write(&file_path, rust_code)?;
        println!("** Generated Rust code: {}", file_path);
        
        self.update_lib_rs(name)?;
        
        let variants = vec![
            FunctionVariant {
                parameter_count: 1,
                parameter_pattern: "count".to_string(),
                rust_function_name: format!("{}_count", name.to_lowercase()),
            },
            FunctionVariant {
                parameter_count: 2,
                parameter_pattern: "range".to_string(),
                rust_function_name: format!("{}_range", name.to_lowercase()),
            },
            FunctionVariant {
                parameter_count: 3,
                parameter_pattern: "step".to_string(),
                rust_function_name: format!("{}_step", name.to_lowercase()),
            },
        ];
        
        Ok(BuiltFunction {
            name: name.to_string(),
            variants,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
        })
    }
    
    fn generate_loop_code(&self, name: &str) -> Result<String> {
        let name_lower = name.to_lowercase();
        let code = format!(r#"pub fn {}_count(count: u32, body: &str) {{
    println!("-- Executing count-based loop: {{}} iterations", count);
    for i in 0..count {{
        println!("  Iteration {{}}: {{}}", i, body);
    }}
}}

pub fn {}_range(start: u32, end: u32, body: &str) {{
    println!("-- Executing range-based loop: {{}} to {{}}", start, end);
    for i in start..end {{
        println!("  Iteration {{}}: {{}}", i, body);
    }}
}}

pub fn {}_step(start: u32, end: u32, step: u32, body: &str) {{
    println!("-- Executing step-based loop: {{}} to {{}} by {{}}", start, end, step);
    let mut i = start;
    while i < end {{
        println!("  Iteration {{}}: {{}}", i, body);
        i += step;
    }}
}}

pub fn {}_condition(condition: &str, body: &str) {{
    println!("-- Executing condition-based loop: while {{}}", condition);
    println!("  Would execute while condition is true: {{}}", body);
}}
"#,
            name_lower,
            name_lower,
            name_lower,
            name_lower
        );

        Ok(code)
    }
    
    fn build_conditional_function(&self, _name: &str) -> Result<BuiltFunction> {
        
        todo!("Conditional function generation not yet implemented")
    }
    
    fn update_lib_rs(&self, function_name: &str) -> Result<()> {
        let lib_rs_path = format!("{}/src/lib.rs", self.functions_dir);
        let mut content = fs::read_to_string(&lib_rs_path)?;
        
        let module_line = format!("pub mod {};", function_name.to_lowercase());
        let use_line = format!("pub use {}::*;", function_name.to_lowercase());
        
        if !content.contains(&module_line) {
            
            if let Some(pos) = content.rfind("pub mod") {
                if let Some(end_pos) = content[pos..].find('\n') {
                    let insert_pos = pos + end_pos + 1;
                    content.insert_str(insert_pos, &format!("{}\n", module_line));
                }
            } else {
                
                content = format!("{}\n\n{}", module_line, content);
            }
        }
        
        if !content.contains(&use_line) {
            if let Some(pos) = content.rfind("pub use") {
                if let Some(end_pos) = content[pos..].find('\n') {
                    let insert_pos = pos + end_pos + 1;
                    content.insert_str(insert_pos, &format!("{}\n", use_line));
                }
            } else {
                
                content.push_str(&format!("\n{}\n", use_line));
            }
        }
        
        fs::write(&lib_rs_path, content)?;
        println!("** Updated lib.rs to include {}", function_name);
        Ok(())
    }
}