@echo off
title Slut Runner
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

REM Run the program with full output to see cache location
cargo run --release -- %SLUT_FILE%
REM cargo run -- %SLUT_FILE%

REM Check if cache folder was created
echo.
echo ========================================
echo   Cache Folder Check
echo ========================================
if exist cache (
    echo Cache folder EXISTS at: %CD%\cache
    echo.
    echo Cache contents:
    dir cache
) else (
    echo WARNING: Cache folder NOT CREATED!
)
echo ========================================

REM Switch to pink for completion
echo.
echo ========================================
echo   Test Complete!
echo ========================================
pause
