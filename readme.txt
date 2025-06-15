# .slut Programming Language

A programming language experiment that lets you specify what you want instead of how to do it. Two implementations exist with different capabilities.

## Project Status

**JavaScript Version (Complete)**: Basic transpiler with mathematical target-seeking
**Rust Version (In Progress)**: Function synthesis and code generation

## What the JavaScript Version Does

- Transpiles `.slut` files to JavaScript and executes them
- **Target-seeking math**: `result([56]) <> randomChoice([1, 2, 3, 55])` finds that 1+55=56
- **Persistent caching**: Remembers solutions to avoid recalculating
- **Function calling**: Classes can call other classes as inputs
- **String interpolation**: `speak("Result: ~variable~")` 
- **Mathematical operations**: Supports 2-number and 3-number equation solving

### JavaScript Example
```slut
* <main> math_test {
   ^ observe_execution {
       result([100]) <> randomChoice([25, 75, 50])
       speak("Found: ~result~")
   }
}
```
System finds 25+75=100 and caches the solution.

## What the Rust Version Does

- **Function synthesis**: `smartLoop(params) <> function(loop)` generates actual Rust code
- **Polymorphic behavior**: Same function adapts to different parameter patterns
  - `smartLoop(5)` = count-based loop  
  - `smartLoop(2,8)` = range-based loop
  - `smartLoop(0,10,2)` = step-based loop with increment
- **Code generation**: Creates real `.rs` files in `functions/src/`
- **Persistent building**: Functions are cached and reused across programs
- **Cross-program sharing**: Functions built by one program available to others

### Rust Example
```slut
* <main> function_builder {
    ^ observe_execution {
        smartLoop(params) <> function(loop)
        smartLoop(3)("console.log('hello')")
    }
}
```
This generates actual Rust code and executes it.

## File Structure

### JavaScript Version
- `quantum_transpiler_unified.js` - Main transpiler
- `quantum_runner_unified.bat` - Interactive runner
- `test_math_A.slut` - Mathematical example
- `quantum_cache.json` - Persistent state storage

### Rust Version  
- `src/main.rs` - Main transpiler
- `src/function_builder.rs` - Code generation
- `src/function_executor.rs` - Function execution
- `functions/src/*.rs` - Generated function library
- `quantum_consciousness_cache.json` - Build cache

## What Still Needs to be Built in Rust

### High Priority
1. **Target-seeking mathematics** from JS version
2. **Mathematical equation solving** (currently missing)
3. **Variable assignment with learning** 
4. **String operations and interpolation**
5. **Conditional function synthesis** (if/else/while)

### Medium Priority
1. **Cross-execution persistence** like JS version
2. **Accuracy improvement system**
3. **Complex mathematical operations** (3-number combinations)
4. **Better error handling**

### Low Priority  
1. **4D mathematical operations** (experimental)
2. **Advanced caching optimization**
3. **Function composition** (functions calling functions)

## Core Concept

Instead of writing:
```rust
for i in 0..5 {
    println!("hello");
}
```

You write:
```slut
smartLoop(5)("console.log('hello')")
```

The system either:
- **JavaScript**: Finds existing solutions that achieve your goal
- **Rust**: Builds new functions that do what you want

## Current Limitations

- **Rust version**: Missing most mathematical capabilities from JS version
- **JavaScript version**: Only simulates execution, doesn't build real code  
- **Both**: Limited to simple examples, no complex program synthesis yet
- **Documentation**: Previous versions were overly promotional

## Usage

**JavaScript**: `node quantum_transpiler_unified.js filename.slut`
**Rust**: `cargo run -- filename.slut`

## Next Steps

1. Port mathematical target-seeking from JavaScript to Rust
2. Implement variable assignment and caching in Rust  
3. Add conditional logic synthesis
4. Combine the best of both versions

The goal is practical: make programming easier by letting you specify intentions instead of detailed instructions.