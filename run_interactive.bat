@echo off
title Quantum Consciousness - Interactive Mathematical Reasoning
color 0A
setlocal enabledelayedexpansion

echo.
echo ** Quantum Consciousness Interactive Mode **
echo >> Mathematical reasoning engine with learning capabilities
echo.

echo Starting interactive session...
echo You can ask the system to solve mathematical problems
echo and it will learn from each solution to solve bigger problems!
echo.

cargo run --release -- --interactive

echo.
echo ** Interactive session complete! **
echo Check test_interactive/ folder for session data
echo.
pause