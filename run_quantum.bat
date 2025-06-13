@echo off
title Quantum Consciousness Runner - Function Building Edition
color 0A
setlocal enabledelayedexpansion

echo.
echo ** Quantum Consciousness Observer - Function Building Edition **
echo >> Building real code from your intentions
echo.

set count=0
for %%f in (*.slut) do (
    set /a count+=1
    echo !count!. %%f
    set file!count!=%%f
)

if %count%==0 (
    echo No .slut files found! 
    pause
    exit /b
)

echo.
set /p choice="Select file (1-%count%): "

if %choice% gtr %count% goto invalid
if %choice% lss 1 goto invalid

call set selectedfile=%%file%choice%%%

echo.
set /p observations="Observations (1-10, default=1): "
if "%observations%"=="" set observations=1

echo.
echo >> Building and executing: %selectedfile% with %observations% observations
echo >> Functions will be generated in the functions/ folder
echo.

cargo run --release -- %selectedfile% -o %observations%

echo.
if exist functions\ (
    echo ** Function library status:
    if exist functions\src\lib.rs (
        echo    - Core library: BUILT
    )
    for %%f in (functions\src\*.rs) do (
        echo    - %%~nf: GENERATED
    )
    echo.
    echo >> Next run will use built functions for improved performance!
)

echo ** Complete!
pause
exit /b

:invalid
echo Invalid choice!
pause