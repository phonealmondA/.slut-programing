use anyhow::Result;

pub struct LoopExecutor {
    // Track loop depth for nested loops
    pub loop_depth: usize,
    // Track if we should break out of loop
    pub should_break: bool,
    // Track if we should continue to next iteration
    pub should_continue: bool,
}

impl LoopExecutor {
    pub fn new() -> Self {
        Self {
            loop_depth: 0,
            should_break: false,
            should_continue: false,
        }
    }

    /// Execute a count-based loop
    pub fn execute_count_loop<F>(
        &mut self,
        count: u32,
        mut body_executor: F
    ) -> Result<()>
    where
        F: FnMut(Option<u32>) -> Result<()>
    {
        println!("-- Executing count loop: {} iterations", count);
        self.loop_depth += 1;

        for i in 0..count {
            // Reset continue flag for each iteration
            self.should_continue = false;

            // Execute body (no loop variable in Phase 1)
            body_executor(None)?;

            // Check for break
            if self.should_break {
                println!("   Loop broken at iteration {}", i);
                self.should_break = false;
                break;
            }

            // Continue already handled by flag reset
        }

        self.loop_depth -= 1;
        println!("-- Count loop complete");
        Ok(())
    }

    /// Execute a range-based loop with iterator variable
    pub fn execute_range_loop<F>(
        &mut self,
        start: i32,
        end: i32,
        mut body_executor: F
    ) -> Result<()>
    where
        F: FnMut(Option<i32>) -> Result<()>
    {
        println!("-- Executing range loop: {} to {}", start, end);
        self.loop_depth += 1;

        for i in start..end {
            self.should_continue = false;

            // Execute body with loop variable
            body_executor(Some(i))?;

            if self.should_break {
                println!("   Loop broken at value {}", i);
                self.should_break = false;
                break;
            }
        }

        self.loop_depth -= 1;
        println!("-- Range loop complete");
        Ok(())
    }

    /// Execute a while loop with condition
    pub fn execute_while_loop<F, C>(
        &mut self,
        mut condition_checker: C,
        mut body_executor: F,
        max_iterations: u32
    ) -> Result<()>
    where
        F: FnMut() -> Result<()>,
        C: FnMut() -> Result<bool>
    {
        println!("-- Executing while loop (max iterations: {})", max_iterations);
        self.loop_depth += 1;

        let mut iteration = 0;
        while condition_checker()? {
            self.should_continue = false;

            body_executor()?;

            if self.should_break {
                println!("   While loop broken at iteration {}", iteration);
                self.should_break = false;
                break;
            }

            iteration += 1;
            if iteration >= max_iterations {
                println!("!! While loop hit max iterations ({})", max_iterations);
                break;
            }
        }

        self.loop_depth -= 1;
        println!("-- While loop complete after {} iterations", iteration);
        Ok(())
    }

    /// Signal that we should break out of the current loop
    pub fn signal_break(&mut self) {
        if self.loop_depth > 0 {
            println!("   >> Break signaled");
            self.should_break = true;
        } else {
            println!("!! Break called outside of loop");
        }
    }

    /// Signal that we should continue to next iteration
    pub fn signal_continue(&mut self) {
        if self.loop_depth > 0 {
            println!("   >> Continue signaled");
            self.should_continue = true;
        } else {
            println!("!! Continue called outside of loop");
        }
    }

    /// Check if we should skip the rest of this iteration
    pub fn should_skip_iteration(&self) -> bool {
        self.should_continue
    }

    /// Check if we're currently inside a loop
    pub fn is_in_loop(&self) -> bool {
        self.loop_depth > 0
    }
}
