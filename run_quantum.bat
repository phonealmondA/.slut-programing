@echo off
title Quantum Consciousness Runner - Rust Edition
color 0A

echo ** Quantum Consciousness Observer (Rust Edition)
echo.

if "%1"=="" (
    echo Usage: run_quantum.bat filename.slut [observations]
    pause
    exit /b
)

set observations=1
if not "%2"=="" set observations=%2

echo >> Executing: %1 with %observations% observations
echo.

cargo run --release -- %1 -o %observations%

echo.
echo ** Complete!
pause