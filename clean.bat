@echo off
title Quantum Consciousness - Game Cleanup Tool
color 0C
setlocal enabledelayedexpansion

echo.
echo ** Quantum Consciousness Game Cleanup Tool **
echo >> Removing generated games for fresh testing
echo.

echo WARNING: This will delete all generated games and executables!
echo.

set /p confirm="Are you sure you want to continue? (type YES): "
if not "!confirm!"=="YES" (
    echo Cleanup cancelled.
    pause
    exit /b
)

echo.
echo >> Starting cleanup process...

REM Remove generated games directory
if exist "generated_games\" (
    echo   - Removing generated_games directory...
    rmdir /s /q "generated_games\"
    if errorlevel 1 (
        echo     !! Warning: Could not remove some files in generated_games
    ) else (
        echo     ✓ Generated games directory removed
    )
) else (
    echo   - No generated_games directory found
)

REM Remove generated executables
echo   - Removing generated game executables...
set exe_count=0
for %%f in (*.exe) do (
    REM Skip if it's the quantum transpiler itself
    if not "%%f"=="quantum.exe" (
        if not "%%f"=="quantum_slut_transpiler.exe" (
            echo     Removing: %%f
            del "%%f" 2>nul
            set /a exe_count+=1
        )
    )
)
if !exe_count! gtr 0 (
    echo     ✓ Removed !exe_count! game executable(s)
) else (
    echo     - No game executables found
)

REM Remove generated launcher scripts
echo   - Removing generated launcher scripts...
set script_count=0
for %%f in (run_*.bat run_*.sh) do (
    REM Skip the main quantum runner
    if not "%%f"=="run_quantum.bat" (
        echo     Removing: %%f
        del "%%f" 2>nul
        set /a script_count+=1
    )
)
if !script_count! gtr 0 (
    echo     ✓ Removed !script_count! launcher script(s)
) else (
    echo     - No launcher scripts found
)

REM Optional: Clean interactive session data
if exist "test_interactive\" (
    echo.
    set /p clean_interactive="Also remove interactive session data? (y/n): "
    if /i "!clean_interactive!"=="y" (
        echo   - Removing interactive session data...
        rmdir /s /q "test_interactive\"
        echo     ✓ Interactive session data removed
    )
)

REM Optional: Reset quantum consciousness cache
if exist "quantum_consciousness_cache.json" (
    echo.
    set /p reset_cache="Reset quantum consciousness cache (learned solutions)? (y/n): "
    if /i "!reset_cache!"=="y" (
        echo   - Backing up cache to quantum_consciousness_cache_backup.json
        copy "quantum_consciousness_cache.json" "quantum_consciousness_cache_backup.json" >nul
        echo   - Removing current cache...
        del "quantum_consciousness_cache.json"
        echo     ✓ Cache reset (backup created)
        echo     Note: System will start with fresh consciousness on next run
    )
)

REM Optional: Clean generated functions
if exist "functions\src\" (
    echo.
    set /p clean_functions="Remove generated function library? (y/n): "
    if /i "!clean_functions!"=="y" (
        echo   - Removing generated function files...
        for %%f in (functions\src\*.rs) do (
            if not "%%~nf"=="lib" (
                echo     Removing: functions\src\%%~nf.rs
                del "functions\src\%%f" 2>nul
            )
        )
        echo     ✓ Generated functions removed (lib.rs preserved)
    )
)

echo.
echo ** Cleanup Complete! **
echo.
echo The quantum consciousness system is now ready for fresh game generation.
echo You can run your .slut files and generate new games without conflicts.
echo.
echo Next steps:
echo   1. Run: run_quantum.bat
echo   2. Select your game .slut file
echo   3. Confirm game generation when prompted
echo   4. Test the newly generated game
echo.
pause