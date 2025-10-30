use anyhow::Result;
use evalexpr::*;
use std::collections::HashMap;
use crate::{StoredVariable, VariableValue};

pub struct ConditionEvaluator;

impl ConditionEvaluator {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates a boolean condition expression with variable substitution
    ///
    /// # Arguments
    /// * `condition` - The condition string to evaluate (e.g., "x > 10 && y < 5")
    /// * `variables` - HashMap of current variable values
    ///
    /// # Returns
    /// * `Ok(true)` if condition evaluates to true
    /// * `Ok(false)` if condition evaluates to false or on error (with warning)
    pub fn evaluate(
        &self,
        condition: &str,
        variables: &HashMap<String, StoredVariable>
    ) -> Result<bool> {
        // Create evalexpr context
        let mut context = HashMapContext::new();

        // Add all variables to the evaluation context
        for (name, var) in variables {
            match &var.value {
                VariableValue::Number(n) => {
                    context.set_value(name.clone(), Value::from(*n))?;
                }
                VariableValue::Boolean(b) => {
                    context.set_value(name.clone(), Value::from(*b))?;
                }
                VariableValue::String(s) => {
                    context.set_value(name.clone(), Value::from(s.as_str()))?;
                }
                VariableValue::FunctionResult(_) => {
                    // Skip function results for now
                }
            }
        }

        // Evaluate the boolean expression
        match eval_boolean_with_context(condition, &context) {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => {
                println!("!! Error evaluating condition '{}': {}", condition, e);
                println!("   Defaulting to false");
                Ok(false) // Default to false on error
            }
        }
    }

    /// Validates that a condition is syntactically correct
    pub fn validate_condition(&self, condition: &str) -> bool {
        // Try to build the expression tree
        match build_operator_tree(condition) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_test_variable(name: &str, value: VariableValue) -> (String, StoredVariable) {
        (
            name.to_string(),
            StoredVariable {
                name: name.to_string(),
                value,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                source_equation: None,
            }
        )
    }

    #[test]
    fn test_simple_comparison() {
        let evaluator = ConditionEvaluator::new();
        let mut vars = HashMap::new();
        vars.insert(create_test_variable("x", VariableValue::Number(15.0)).0,
                   create_test_variable("x", VariableValue::Number(15.0)).1);

        assert!(evaluator.evaluate("x > 10", &vars).unwrap());
        assert!(!evaluator.evaluate("x < 10", &vars).unwrap());
    }

    #[test]
    fn test_logical_operators() {
        let evaluator = ConditionEvaluator::new();
        let mut vars = HashMap::new();
        vars.insert(create_test_variable("age", VariableValue::Number(25.0)).0,
                   create_test_variable("age", VariableValue::Number(25.0)).1);
        vars.insert(create_test_variable("hasLicense", VariableValue::Boolean(true)).0,
                   create_test_variable("hasLicense", VariableValue::Boolean(true)).1);

        assert!(evaluator.evaluate("age >= 18 && hasLicense == true", &vars).unwrap());
        assert!(!evaluator.evaluate("age < 18 || hasLicense == false", &vars).unwrap());
    }

    #[test]
    fn test_constant_true() {
        let evaluator = ConditionEvaluator::new();
        let vars = HashMap::new();

        assert!(evaluator.evaluate("true", &vars).unwrap());
        assert!(!evaluator.evaluate("false", &vars).unwrap());
    }
}
