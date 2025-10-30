@echo off
title Quantum Consciousness Test Runner
setlocal enabledelayedexpansion

REM ============================================================================
REM CONFIGURATION - Change the file to run here
REM ============================================================================
REM To run a different .slut file, change the line below:
set SLUT_FILE=test_while_loop.slut

REM Other available test files:
REM set SLUT_FILE=test_count_loop.slut
REM set SLUT_FILE=test_range_loop.slut
REM set SLUT_FILE=test_while_loop.slut
REM set SLUT_FILE=test_loop_control.slut
REM set SLUT_FILE=test_all_loops_working.slut
REM set SLUT_FILE=test_nested_simple.slut
REM ============================================================================

REM Pink color for header
echo.
echo ========================================
echo   Quantum Consciousness Test Runner
echo ========================================
echo.
echo Running: %SLUT_FILE%
echo.

REM Switch to light blue for execution

REM Run the program with clean output (filters out warnings and build messages)
cargo run --quiet --release -- %SLUT_FILE% 2>&1 | findstr /V /C:"warning:" /C:"-->" /C:"|" /C:"Compiling" /C:"Finished" /C:"Running" /C:"target\release"

REM Switch to pink for completion
echo.
echo ========================================
echo   Test Complete!
echo ========================================
pause
